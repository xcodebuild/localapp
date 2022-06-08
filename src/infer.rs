use std::{fs::File, path::Path};

use download_rs::sync_download::Download;
use log::{info, trace, warn};
use regex::Regex;
use reqwest;
use sanitize_html::{rules::predefined::DEFAULT, sanitize_str};
use site_icons::{IconKind, Icons};
use tokio::task::spawn_blocking;

use crate::consts::{TEMP_DIR, USER_AGENT_REQUEST};

pub async fn get_content(url: String) -> String {
    let content = reqwest::Client::builder()
        // .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return content;
}

pub async fn infer_title(url: String) -> String {
    let content = get_content(url).await;
    let re = Regex::new(r"<title>(.*?)</title>").unwrap();
    let result = re.find(&content).unwrap().as_str().to_string();
    let sanitized_result: String = sanitize_str(&DEFAULT, &result).unwrap();
    return sanitized_result.to_string();
}

pub async fn infer_icon(url: String) -> String {
    let mut icons = Icons::new();
    // scrape the icons from a url
    icons.load_website(&(url)).await;
    let entries = icons.entries().await;

    let icon = {
        let mut icon_str = entries[0].url.to_string();
        let mut size = 0;
        for entry in entries {
            let height = if entry.info.size().is_none() {
                0
            } else {
                entry.info.size().unwrap().height
            };
            if entry.kind == IconKind::AppIcon && (size == 0 || height > size) {
                icon_str = entry.url.to_string();
                size = height;
            }
        }
        icon_str
    };

    println!("Icon: {:?}", &icon);

    let icon_path = Path::new(&TEMP_DIR.as_ref()).join("icon.png");
    let icon_path_string = icon_path.as_path().to_str().unwrap();

    let mut file = std::fs::File::create(icon_path_string).unwrap();

    spawn_blocking(move || {
        reqwest::blocking::get(icon)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();
    })
    .await
    .unwrap();

    return icon_path_string.to_string();
}

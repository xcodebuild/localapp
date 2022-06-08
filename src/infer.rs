use std::{fs::File, path::Path};

use download_rs::sync_download::Download;
use log::{info, trace, warn};
use regex::Regex;
use reqwest;
use sanitize_html::{rules::predefined::DEFAULT, sanitize_str};
use site_icons::{IconKind, Icons};
use tokio::task::spawn_blocking;
use website_icon_extract::ImageLink;
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
    let find_result = re.find(&content);
    let result = if find_result.is_none() {
        "LocalApp".to_string()
    } else {
        find_result.unwrap().as_str().to_string()
    };
    let sanitized_result: String = sanitize_str(&DEFAULT, &result).unwrap();
    return sanitized_result.to_string();
}

pub async fn infer_icon(url: String) -> String {
    let mut icons = Icons::new();
    // scrape the icons from a url
    icons.load_website(&(url)).await;
    let entries = icons.entries().await;

    let url_str = url.clone();
    let icon_list = spawn_blocking(move || {ImageLink::from_website(url_str, USER_AGENT_REQUEST, 5).unwrap()}).await.unwrap();

    let icon: String = if icon_list.len() > 0 {
        let iconUrl = &icon_list[0].url;
        iconUrl.to_string()
    } else {
        let icon_fallback = {
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
        icon_fallback
    };

    

    println!("Icon: {:?}", &icon);

    let icon_path = Path::new(&TEMP_DIR.path()).join("icon.png");
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

use std::{fs::File, path::Path};

use futures::io;
use reqwest;
use regex::Regex;
use sanitize_html::{sanitize_str, rules::predefined::DEFAULT};
use tokio::task::spawn_blocking;
use download_rs::sync_download::Download;
use log::{info, trace, warn};
use substring::Substring;
use site_icons::Icons;


use crate::{consts::{USER_AGENT_REQUEST, TEMP_DIR}};

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
    icons.load_website(&url).await;
    let entries = icons.entries().await;
    let icon = &entries[0].url.to_string();
    let icon_path = Path::new(&TEMP_DIR.as_ref()).join("icon.png");
    let icon_path_string = icon_path.as_path().to_str();
    let download = Download::new(icon, icon_path_string,None);

    match download.download() {
        Ok(_) => info!("Download icon success: {:?}", icon_path_string),
        Err(e) => info!("Download error: {}",e.to_string()),
    }
    return icon_path_string.unwrap().to_string();
}


use anyhow::Result;
use axum::extract::Path;
use reqwest::get;
use serde::Deserialize;
use std::{fmt::Display, fs::create_dir_all, io::Cursor, path::Path as StdPath};
use tokio::{fs::File, io::copy};

use crate::{constants::config::CONFIG_JSON_PATH, utils::json::read_json};

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub hash: String,
}

impl Asset {
    async fn download_file<T: Display + PartialEq<&'static str> + AsRef<StdPath>>(&self, mode: T, path: T) -> Result<()> {
        let url = if mode == "cn" {
            format!(
                "https://ak.hycdn.cn/assetbundle/official/Android/assets/{}/{}",
                self.hash, self.name
            )
        } else {
            format!(
                "https://ark-us-static-online.yo-star.com/assetbundle/official/Android/assets/{}/{}",
                self.hash, self.name
            )
        };
        let response = get(&url).await?;
        let path = format!("{}/{}", path, self.name);
        let mut file = File::create(&path).await?;
        let mut cursor = Cursor::new(response.bytes().await?);
        copy(&mut cursor, &mut file).await?;
        Ok(())
    }
}

pub async fn get_file(Path(asset): Path<Asset>) {
    let hash = &asset.hash;
    let config = read_json(CONFIG_JSON_PATH);
    let mode = config["server"]["mode"].as_str().unwrap();

    if config["assets"]["downloadLocally"].as_bool().unwrap() {
        if StdPath::new(&format!("./assets/{hash}/redirect/")).exists() {
            create_dir_all(format!("./assets/{hash}/redirect/")).unwrap();
        }
        asset.download_file(mode, &format!("./assets/{hash}/redirect/")).await.unwrap();
    }
}

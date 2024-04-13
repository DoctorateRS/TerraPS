use anyhow::Result;
use axum::extract::Path;
use reqwest::get;
use serde::Deserialize;
use std::io::Cursor;
use tokio::{fs::File, io::copy};

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub hash: String,
}

impl Asset {
    async fn download_file(&self, mode: &str, path: &str) -> Result<()> {
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
        let mut file = File::create(path).await?;
        let mut cursor = Cursor::new(response.bytes().await?);
        copy(&mut cursor, &mut file).await?;
        Ok(())
    }
}

pub async fn get_file(Path(asset): Path<Asset>) {}

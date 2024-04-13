use axum::extract::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub hash: String,
}

impl Asset {
    async fn download_file(&self, mode: &str, path: &str) -> Result<Vec<u8>, reqwest::Error> {
        let url = format!("https://example.com/{}", self.name);
        let response = reqwest::get(&url).await?;
        response.bytes().await.map(|b| b.to_vec())
    }
}

pub async fn get_file(Path(asset): Path<Asset>) {}

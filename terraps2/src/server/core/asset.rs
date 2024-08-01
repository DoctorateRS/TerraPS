use std::{io::Cursor, path::Path};

use anyhow::Result;
use axum::{body::Body, response::IntoResponse, Json};
use common_utils::read_json;
use reqwest::get;
use tokio::{fs::File, io::copy};
use tokio_util::io::ReaderStream;

const BASE_PATH_CN: &str = "https://ak.hycdn.cn/assetbundle/official/Android/assets";
const BASE_PATH_GL: &str = "https://ark-us-static-online.yo-star.com/assetbundle/official/Android/assets";
const ASSETS_JSON: &str = "./assets/assets.json";

use crate::SERVER_CONFIG;

pub struct Asset {
    pub name: String,
    pub hash: String,
}

impl Asset {
    fn url(&self) -> String {
        let base_path = if SERVER_CONFIG.mode == "cn" { BASE_PATH_CN } else { BASE_PATH_GL };
        format!("{}/{}/{}", base_path, self.hash, self.name)
    }

    async fn download_asset<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let url = self.url();
        let response = get(url).await?;

        let path = format!("{}/{}", path.as_ref().to_str().unwrap_or("."), self.name);
        let mut file = File::create(path).await?;
        let mut resp = Cursor::new(response.bytes().await?);

        copy(&mut resp, &mut file).await?;

        Ok(())
    }

    async fn query_local(&self) -> impl IntoResponse {
        let path = format!("./assets/{hash}/redirect/{name}", hash = &self.hash, name = &self.name);
        let file = File::open(path).await.unwrap();
        let stream = ReaderStream::new(file);
        Body::from_stream(stream)
    }

    async fn query_hot_update_list(&self) -> impl IntoResponse {
        let path = format!("./assets/{hash}/redirect/{name}", hash = &self.hash, name = &self.name);
        let hot_update = read_json(path);
        Json(hot_update)
    }
}

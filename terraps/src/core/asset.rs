use anyhow::{Ok, Result};
use axum::{
    body::Body,
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use reqwest::get;
use serde::Deserialize;
use serde_json::Value;
use std::{fmt::Display, fs::create_dir_all, io::Cursor, path::Path as StdPath};
use tokio::{fs::File, io::copy};
use tokio_util::io::ReaderStream;

use crate::constants::config::CONFIG_JSON_PATH;
use common_utils::{read_json, write_json};

const BASE_PATH_CN: &str = "https://ak.hycdn.cn/assetbundle/official/Android/assets/";
const BASE_PATH_GL: &str = "https://ark-us-static-online.yo-star.com/assetbundle/official/Android/assets/";

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub hash: String,
}

impl Asset {
    async fn download_file<T: Display + PartialEq<&'static str> + AsRef<StdPath>>(&self, mode: T, path: T) -> Result<()> {
        let url = if mode == "cn" {
            format!("{BASE_PATH_CN}{}/{}", self.hash, self.name)
        } else {
            format!("{BASE_PATH_GL}/{}/{}", self.hash, self.name)
        };
        let response = get(&url).await?;
        let path = format!("{}/{}", path, self.name);
        let mut file = File::create(&path).await?;
        let mut cursor = Cursor::new(response.bytes().await?);
        copy(&mut cursor, &mut file).await?;
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
        let hot_update = read_json(&path);
        Json(hot_update)
    }
}

pub async fn get_file(Path(asset): Path<Asset>) -> Response {
    let name = &asset.name;
    let hash = &asset.hash;
    let path = format!("./assets/{hash}/redirect/");
    let config = read_json(CONFIG_JSON_PATH);
    let mode = config["server"]["mode"].as_str().unwrap();
    if config["assets"]["downloadLocally"].as_bool().unwrap() {
        if !StdPath::new(&path).exists() {
            create_dir_all(&path).unwrap();
        }
        asset.download_file(mode, &path).await.unwrap();
    } else {
        todo!("Redirect not supported yet.");
    }
    if name == "hot_update_list.json" {
        let hot_update_list = get(format!("{BASE_PATH_CN}{hash}/hot_update_list.json", hash = &hash))
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
        write_json(&format!("{path}hot_update_list.json"), &hot_update_list);
        asset.query_hot_update_list().await.into_response()
    } else {
        asset.query_local().await.into_response()
    }
}

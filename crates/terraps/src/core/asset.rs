use anyhow::Result;
use axum::{
    body::Body,
    extract::Path,
    http::{HeaderName, HeaderValue},
    response::{IntoResponse, Response},
    Json,
};
use reqwest::{get, StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{fmt::Display, fs::create_dir_all, io::Cursor, path::Path as StdPath};
use tokio::{fs::File, io::copy};
use tokio_util::io::ReaderStream;

use crate::constants::config::CONFIG_JSON_PATH;
use common_utils::{read_json, write_json};

const BASE_PATH_CN: &str = "https://ak.hycdn.cn/assetbundle/official/Android/assets/";
const BASE_PATH_GL: &str = "https://ark-us-static-online.yo-star.com/assetbundle/official/Android/assets/";
const ASSETS_JSON: &str = "./assets/assets.json";

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub hash: String,
}

impl Asset {
    async fn download_file<T: Display + PartialEq<&'static str>, P: AsRef<StdPath>>(&self, mode: T, path: P) -> Result<()> {
        let url = self.get_url(mode);
        let response = get(&url).await?;
        let path = format!("{}/{}", path.as_ref().to_str().unwrap_or(""), self.name);
        let mut file = File::create(&path).await?;
        let mut cursor = Cursor::new(response.bytes().await?);
        copy(&mut cursor, &mut file).await?;
        Ok(())
    }
    fn get_url<T: PartialEq<&'static str>>(&self, mode: T) -> String {
        if mode == "cn" {
            format!("{BASE_PATH_CN}{}/{}", self.hash, self.name)
        } else {
            format!("{BASE_PATH_GL}/{}/{}", self.hash, self.name)
        }
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

pub async fn get_file(Path(asset): Path<Asset>) -> Response {
    let name = &asset.name;
    let hash = &asset.hash;
    let path = format!("./assets/{hash}/redirect/");
    let config = read_json(CONFIG_JSON_PATH);
    let mode = config["server"]["mode"].as_str().unwrap();
    let mut assets_list = if !StdPath::new(ASSETS_JSON).exists() { json!({hash: []}) } else { read_json(ASSETS_JSON) };
    if assets_list.get(hash).is_none() {
        assets_list[hash] = json!([]);
    }
    let contains_asset = assets_list[hash].as_array().unwrap().contains(&json!(name));
    if !contains_asset {
        assets_list[hash].as_array_mut().unwrap().push(json!(name));
    }
    write_json(ASSETS_JSON, &assets_list).unwrap_or(());
    if config["assets"]["downloadLocally"].as_bool().unwrap() {
        if !StdPath::new(&path).exists() {
            create_dir_all(&path).unwrap();
        }
        asset.download_file(mode, &path).await.unwrap();
    } else {
        let res = match get(asset.get_url(mode)).await {
            Ok(res) => res,
            Err(_) => return (StatusCode::BAD_REQUEST, Body::empty()).into_response(),
        };
        let mut builder = Response::builder().status(res.status().as_u16());
        let headers = res
            .headers()
            .into_iter()
            .map(|(n, v)| {
                let name = HeaderName::from_bytes(n.as_ref()).unwrap();
                let val = HeaderValue::from_bytes(v.as_bytes()).unwrap();
                (name, val)
            })
            .collect::<Vec<_>>();
        for (name, val) in headers {
            builder = builder.header(name, val);
        }
        return builder.body(Body::from_stream(res.bytes_stream())).unwrap();
    }
    if name == "hot_update_list.json" {
        let hot_update_list = get(format!("{BASE_PATH_CN}{hash}/hot_update_list.json", hash = &hash)).await.unwrap().json::<Value>().await.unwrap();
        write_json(format!("{path}hot_update_list.json"), &hot_update_list).unwrap_or(());
        asset.query_hot_update_list().await.into_response()
    } else {
        asset.query_local().await.into_response()
    }
}

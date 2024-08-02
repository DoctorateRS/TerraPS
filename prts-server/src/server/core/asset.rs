use std::{collections::HashMap, io::Cursor, path::Path as OsPath};

use anyhow::Result;
use axum::{
    body::Body,
    extract::Path,
    http::{HeaderName, HeaderValue},
    response::{IntoResponse, Response},
    Json,
};
use common_utils::{read_json, write_json};
use reqwest::{get, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use tokio::{
    fs::{create_dir_all, File},
    io::copy,
};
use tokio_util::io::ReaderStream;

const BASE_PATH_CN: &str = "https://ak.hycdn.cn/assetbundle/official/Android/assets";
const BASE_PATH_GL: &str = "https://ark-us-static-online.yo-star.com/assetbundle/official/Android/assets";
const ASSETS_JSON: &str = "./assets/assets.json";

use crate::{ASSET_CONFIG, SERVER_CONFIG};

type AssetList = HashMap<String, Vec<String>>;

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub hash: String,
}

impl Asset {
    fn url(&self) -> String {
        let base_path = if SERVER_CONFIG.mode == "cn" { BASE_PATH_CN } else { BASE_PATH_GL };
        format!("{}/{}/{}", base_path, self.hash, self.name)
    }

    async fn download_asset<P: AsRef<OsPath>>(&self, path: P) -> Result<()> {
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

pub async fn get_file(Path(asset): Path<Asset>) -> Response {
    let path = format!("./assets/{}/redirect/", &asset.hash);

    let mut asset_list = from_value::<AssetList>(read_json(ASSETS_JSON)).unwrap_or_else(|_| HashMap::new());

    if !asset_list.contains_key(&asset.hash) {
        asset_list.insert(asset.hash.clone(), vec![]);
    }

    asset_list.get_mut(&asset.hash).unwrap().push(asset.name.clone());

    if ASSET_CONFIG.dl_local {
        if !OsPath::new(&path).exists() {
            create_dir_all(&path).await.unwrap();
        }
        asset.download_asset(&path).await.unwrap();
    } else {
        let res = match get(asset.url()).await {
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

    if asset.name == "hot_update_list.json" {
        let hot_update = get(format!("{}/{}/hot_update_list.json", BASE_PATH_CN, &asset.hash)).await.unwrap().json::<Value>().await.unwrap();
        write_json(format!("{}hot_update_list.json", path), &hot_update).unwrap();
        asset.query_hot_update_list().await.into_response()
    } else {
        asset.query_local().await.into_response()
    }
}

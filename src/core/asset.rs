use core::str;

use axum::{extract::Path, Json};

use reqwest::RequestBuilder;
use serde::Deserialize;
use serde_json::json;

use crate::utils::json::JSON;

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub hash: String,
}

pub async fn get_file(Path(Asset): Path<Asset>) -> JSON {
    Json(json!({"name": Asset.name, "hash": Asset.hash}))
}

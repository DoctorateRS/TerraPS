use axum::{extract::Path, Json};

use reqwest::RequestBuilder;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub hash: String,
}

pub async fn get_file(Path(asset): Path<Asset>) {}

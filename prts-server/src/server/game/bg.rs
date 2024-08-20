use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Payload {
    bg_id: String,
}

pub async fn change_bg(Json(payload): Json<Payload>) {}

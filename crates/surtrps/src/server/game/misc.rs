use axum::Json;

use models::{EventResponse, MiscResponse, BATCH_EVENT, EVENT};

pub async fn event<'a>() -> Json<EventResponse<'a>> {
    Json(EVENT)
}

pub async fn batch_event<'a>() -> Json<MiscResponse<'a>> {
    Json(BATCH_EVENT)
}

pub async fn beat<'a>() -> Json<MiscResponse<'a>> {
    Json(BATCH_EVENT)
}

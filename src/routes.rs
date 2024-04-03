use crate::core::prod;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .nest("/config/prod", config_routes())
        .nest("/crisis", crisis_routes())
        .nest("/crisisV2", crisis_v2_routes())
        .fallback(fallback)
}

fn app_routes() -> Router {
    Router::new().route("/", get(|| async { "Hello, world!" }))
}

fn config_routes() -> Router {
    Router::new()
        .route("/official/Android/version", get(prod::prod_android_version))
        .route("/official/network_config", get(prod::prod_network_config))
        .route("/official/remote_config", get(prod::prod_remote_config))
        .route("/official/refresh_config", get(prod::prod_refresh_config))
        .route("/announce_meta/Android/announcement.meta.jsons", get(prod::prod_announcement))
        .route(
            "/announce_meta/Android/preannouncement.meta.json",
            get(prod::prod_pre_announcement),
        )
}

fn crisis_routes() -> Router {
    Router::new().route("/crisis", get(|| async { "Crisis" }))
}

fn crisis_v2_routes() -> Router {
    Router::new().route("/crisisV2", get(|| async { "CrisisV2" }))
}

async fn fallback() -> &'static str {
    "Hello, world!"
}

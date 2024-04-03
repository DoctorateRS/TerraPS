use crate::core::prod;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/config/prod", config_routes())
        .nest("/crisis", crisis_routes())
        .nest("/crisisV2", crisisV2_routes())
}

fn config_routes() -> Router {
    Router::new()
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

fn crisisV2_routes() -> Router {
    Router::new().route("/crisisV2", get(|| async { "CrisisV2" }))
}

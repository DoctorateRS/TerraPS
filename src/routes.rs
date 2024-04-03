use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/config", config_routes())
        .nest("/crisis", crisis_routes())
        .nest("/crisisV2", crisisV2_routes())
}

fn config_routes() -> Router {
    Router::new().route("/config", get(|| async { "Config" }))
}

fn crisis_routes() -> Router {
    Router::new().route("/crisis", get(|| async { "Crisis" }))
}

fn crisisV2_routes() -> Router {
    Router::new().route("/crisisV2", get(|| async { "CrisisV2" }))
}

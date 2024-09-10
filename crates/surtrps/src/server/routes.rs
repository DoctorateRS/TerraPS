use axum::{
    routing::{get, post},
    Json, Router,
};

use tower_http::trace::{DefaultMakeSpan, DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use models::PlayerDataDelta;

use super::{
    core::{asset, prod_cfg},
    game::{account, bg, misc},
};

pub fn app() -> Router {
    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().level(Level::INFO))
        .on_request(DefaultOnRequest::default().level(Level::INFO))
        .on_eos(DefaultOnEos::default().level(Level::INFO))
        .on_failure(DefaultOnFailure::default().level(Level::ERROR))
        .on_response(DefaultOnResponse::default().level(Level::DEBUG));

    Router::new()
        .nest("/config/prod", prod_config_routes())
        .nest("/crisisV2", crisis_v2_routes())
        .nest("/account", account_routes())
        .merge(misc_routes())
        .layer(trace)
        .fallback(fallback)
}

fn account_routes() -> Router {
    Router::new()
        .route("/syncPushMessage", post(account::sync_push_data))
        .route("/login", post(account::login))
        .route("/syncData", post(account::sync_data))
        .route("/syncStatus", post(account::sync_status))
}

fn crisis_v2_routes() -> Router {
    Router::new()
}

fn prod_config_routes() -> Router {
    Router::new()
        .route("/official/Android/version", get(prod_cfg::prod_version))
        .route("/official/network_config", get(prod_cfg::prod_network))
        .route("/official/remote_config", get(prod_cfg::prod_remote))
        .route("/official/refresh_config", get(prod_cfg::prod_refresh))
        .route("/announce_meta/Android/announcement.meta.jsons", get(prod_cfg::prod_announce))
        .route("/announce_meta/Android/preannouncement.meta.json", get(prod_cfg::prod_preannounce))
}

fn misc_routes() -> Router {
    Router::new().route("/assetbundle/official/Android/assets/:hash/:name", get(asset::get_file))
    // .route("/background/setBackground", post(bg::change_bg))
    // .route("/homeTheme/change", post(bg::change_theme))
    // .route("/event", post(misc::event))
    // .route("/batch_event", post(misc::batch_event))
    // .route("/beat", post(misc::beat))
}

async fn fallback() -> Json<PlayerDataDelta> {
    Json(PlayerDataDelta::default())
}

use axum::{
    routing::{get, post},
    Router,
};

use tower_http::trace::{DefaultMakeSpan, DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use super::core::asset;

pub fn app() -> Router {
    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().level(Level::INFO))
        .on_request(DefaultOnRequest::default().level(Level::INFO))
        .on_eos(DefaultOnEos::default().level(Level::INFO))
        .on_failure(DefaultOnFailure::default().level(Level::ERROR))
        .on_response(DefaultOnResponse::default().level(Level::DEBUG));

    Router::new().merge(misc_routes()).layer(trace)
}

fn misc_routes() -> Router {
    Router::new().route("/assetbundle/official/Android/assets/:hash/:name", get(asset::get_file))
}

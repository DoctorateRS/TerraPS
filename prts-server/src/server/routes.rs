use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub fn app() -> Router {
    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().level(Level::INFO))
        .on_request(DefaultOnRequest::default().level(Level::INFO))
        .on_eos(DefaultOnEos::default().level(Level::INFO))
        .on_failure(DefaultOnFailure::default().level(Level::ERROR))
        .on_response(DefaultOnResponse::default().level(Level::DEBUG));
    Router::new().layer(trace)
}

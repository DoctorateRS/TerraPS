use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/", get(|| async { "Hello, world!" }))
}

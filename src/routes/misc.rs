use axum::http::{StatusCode, Uri};
use tracing::debug;

pub(super) async fn unhandled(uri: Uri, body: String) -> StatusCode {
    debug!("Unhandled request: {} {}", uri, body);

    StatusCode::IM_A_TEAPOT
}

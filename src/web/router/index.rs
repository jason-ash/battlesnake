use crate::State;
use axum::{response::IntoResponse, routing::get, Router};

pub fn router() -> Router<State> {
    Router::new().route("/", get(handler))
}

async fn handler() -> impl IntoResponse {
    "Hello world!"
}

use crate::State;
use axum::{response::IntoResponse, routing::get, Router};

pub fn router(state: State) -> Router {
    Router::new().route("/", get(handler)).with_state(state)
}

async fn handler() -> impl IntoResponse {
    "hello world!"
}

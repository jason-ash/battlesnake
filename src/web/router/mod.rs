use crate::State;
use axum::{response::IntoResponse, routing::get, Router};

mod index;

pub fn router(state: State) -> Router {
    Router::new().merge(index::router()).with_state(state)
}

async fn handler() -> impl IntoResponse {
    "hello world!"
}

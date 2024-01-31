use axum::{response::IntoResponse, routing::post, Router};

use crate::State;

pub fn router() -> Router<State> {
    Router::new().route("/move", post(handler))
}

async fn handler() -> impl IntoResponse {
    "move up!"
}

use crate::{model::payload::StartGamePayload, State};
use axum::{http::StatusCode, routing::post, Json, Router};

pub fn router() -> Router<State> {
    Router::new().route("/start", post(handler))
}

async fn handler(Json(_payload): Json<StartGamePayload>) -> StatusCode {
    StatusCode::OK
}

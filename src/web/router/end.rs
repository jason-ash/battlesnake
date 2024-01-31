use crate::{model::EndGamePayload, State};
use axum::{http::StatusCode, routing::post, Json, Router};

pub fn router() -> Router<State> {
    Router::new().route("/end", post(handler))
}

async fn handler(Json(_payload): Json<EndGamePayload>) -> StatusCode {
    StatusCode::OK
}

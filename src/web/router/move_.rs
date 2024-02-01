use axum::{routing::post, Json, Router};

use crate::{
    model::payload::{MoveRequestPayload, MoveResponsePayload},
    State,
};

pub fn router() -> Router<State> {
    Router::new().route("/move", post(handler))
}

async fn handler(Json(payload): Json<MoveRequestPayload>) -> Json<MoveResponsePayload> {
    MoveResponsePayload::default().into()
}

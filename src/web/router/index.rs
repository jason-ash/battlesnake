use crate::{model::InitializePayload, State};
use axum::{routing::get, Json, Router};

pub fn router() -> Router<State> {
    Router::new().route("/", get(handler))
}

async fn handler() -> Json<InitializePayload> {
    let payload = InitializePayload::new(Some("author".to_string()), None, None, None, None);
    Json(payload)
}

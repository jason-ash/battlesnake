use crate::{model::InitializePayload, State};
use axum::{routing::get, Json, Router};

pub fn router() -> Router<State> {
    Router::new().route("/", get(handler))
}

async fn handler() -> Json<InitializePayload> {
    InitializePayload::default().with_author("jason-ash").into()
}

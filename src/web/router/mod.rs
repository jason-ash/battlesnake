mod index;

pub fn router(state: crate::State) -> axum::Router {
    axum::Router::new().merge(index::router()).with_state(state)
}

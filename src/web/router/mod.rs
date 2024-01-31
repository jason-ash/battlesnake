mod end;
mod index;
mod move_;

pub fn router(state: crate::State) -> axum::Router {
    axum::Router::new()
        .merge(index::router())
        .merge(move_::router())
        .merge(end::router())
        .with_state(state)
}

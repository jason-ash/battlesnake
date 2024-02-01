mod end;
mod index;
mod move_;
mod start;

pub fn router(state: crate::State) -> axum::Router {
    axum::Router::new()
        .merge(index::router())
        .merge(start::router())
        .merge(move_::router())
        .merge(end::router())
        .with_state(state)
}

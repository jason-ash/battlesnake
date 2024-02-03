use super::payload::Move;

pub trait SnakeAgent<S> {
    fn make_move(&mut self, state: &S) -> Move;
}

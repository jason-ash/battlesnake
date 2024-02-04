use super::Move;
use std::collections::VecDeque;

pub struct Snake {
    head: usize,
    body: VecDeque<usize>,
}

impl Snake {
    /// return the direction immediately "backwards" of the snake, which,
    /// if selected, would result in an immediate loss by self-elimination.
    pub fn backwards(&self) -> Move {
        todo!()
    }
}

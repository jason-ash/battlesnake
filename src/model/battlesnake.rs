use super::{Cell, Move};
use std::collections::VecDeque;

pub struct BattleSnake {
    head: Cell,
    body: VecDeque<Cell>,
}

impl BattleSnake {
    /// return the direction immediately "backwards" of the snake, which,
    /// if selected, would result in an immediate loss by self-elimination.
    pub fn backwards(&self) -> Move {
        todo!()
    }
}

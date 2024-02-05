use super::{Cell, Move};
use std::collections::VecDeque;

pub struct BattleSnake {
    body: VecDeque<Cell>,
}

impl BattleSnake {
    /// return the direction immediately "backwards" of the snake, which,
    /// if selected, would result in an immediate loss by self-elimination.
    pub fn backwards(&self) -> Move {
        // head is the first element in the body vec; neck is the second.
        let head = self.body.get(0).expect("snake to have a head.");
        let neck = self.body.get(1).expect("snake to have a neck.");

        match head.0 as isize - neck.0 as isize {
            11 => Move::Up,
            1 => Move::Left,
            -1 => Move::Right,
            -11 => Move::Down,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backwards_moves() {
        let cases = [
            ([Cell(10), Cell(9), Cell(8)], Move::Left),
            ([Cell(32), Cell(21), Cell(20)], Move::Up),
            ([Cell(10), Cell(21), Cell(20)], Move::Down),
            ([Cell(0), Cell(1), Cell(2)], Move::Right),
        ];
        for (cells, move_) in cases {
            let body = VecDeque::from(cells);
            let battlesnake = BattleSnake { body };
            assert!(matches!(battlesnake.backwards(), move_));
        }
    }
}

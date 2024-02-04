use super::Cell;

/// position on the gameboard from the battlesnake server, numbered
/// using (x, y) coordinates starting with zero on the bottom left.
#[derive(Debug, serde::Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<Cell> for Position {
    fn from(value: Cell) -> Self {
        let x = value.0 % 11;
        let y = value.0 / 11;
        Position { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_from_cell() {
        let cases = [
            (Position { x: 0, y: 0 }, Cell(110)),
            (Position { x: 10, y: 0 }, Cell(120)),
            (Position { x: 0, y: 10 }, Cell(0)),
            (Position { x: 10, y: 10 }, Cell(10)),
        ];
        for (position, cell) in cases {
            assert!(matches!(Position::from(cell), position));
        }
    }
}

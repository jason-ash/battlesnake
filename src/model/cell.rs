use super::Position;

#[derive(Debug)]
pub struct Cell(pub usize);

impl From<Position> for Cell {
    fn from(value: Position) -> Self {
        let col = value.x;
        let row = 10 - value.y;
        let idx = row * 11 + col;
        Cell(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_from_position() {
        let cases = [
            (Cell(110), Position { x: 0, y: 0 }),
            (Cell(120), Position { x: 10, y: 0 }),
            (Cell(0), Position { x: 0, y: 10 }),
            (Cell(10), Position { x: 10, y: 10 }),
        ];
        for (cell, position) in cases {
            assert!(matches!(Cell::from(position), cell));
        }
    }
}

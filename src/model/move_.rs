#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Move {
    fn default() -> Self {
        Self::Up
    }
}

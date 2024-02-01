use super::{Battlesnake, Position};

#[derive(Debug, serde::Deserialize)]
pub struct Board {
    pub height: u8,
    pub width: u8,
    pub food: Vec<Position>,
    pub hazards: Vec<Position>,
    pub snakes: Vec<Battlesnake>,
}

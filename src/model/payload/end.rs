use super::{Battlesnake, Board, Game};

#[derive(Debug, serde::Deserialize)]
pub struct EndGamePayload {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}

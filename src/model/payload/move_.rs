use crate::model::{battlesnake::Battlesnake, board::Board, game::Game};

#[derive(Debug, serde::Deserialize)]
pub struct MoveRequestPayload {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Debug, serde::Serialize)]
pub struct MoveResponsePayload {
    #[serde(rename = "move")]
    pub move_: Move,
    pub shout: String,
}

#[derive(Debug, serde::Serialize)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

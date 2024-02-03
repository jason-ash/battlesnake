use super::{Battlesnake, Board, Game};
use crate::model::Move;

#[derive(Debug, serde::Deserialize)]
pub struct MoveRequestPayload {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct MoveResponsePayload {
    #[serde(rename = "move")]
    pub move_: Move,
    pub shout: String,
}

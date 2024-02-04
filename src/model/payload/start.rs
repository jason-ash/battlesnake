use super::{Battlesnake, Board, Game};

#[derive(Debug, serde::Deserialize)]
pub struct StartGamePayload {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

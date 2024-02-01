use super::{Customizations, Position};
use std::collections::VecDeque;

#[derive(Debug, serde::Deserialize)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: u8,
    pub body: VecDeque<Position>,
    pub latency: u32,
    pub head: Position,
    pub length: u8,
    pub shout: String,
    pub squad: String,
    pub customizations: Customizations,
}

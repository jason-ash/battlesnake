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

#[derive(Debug, serde::Deserialize)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct Customizations {
    pub color: String,
    pub head: String,
    pub tail: String,
}

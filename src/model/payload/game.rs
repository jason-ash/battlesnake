use super::Ruleset;

#[derive(Debug, serde::Deserialize)]
pub struct Game {
    pub id: String,
    pub ruleset: Ruleset,
    pub timeout: u32,
    pub map: Option<String>,
    pub source: Option<String>,
}

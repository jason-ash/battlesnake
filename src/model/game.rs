#[derive(Debug, serde::Deserialize)]
pub struct Game {
    pub id: String,
    pub ruleset: Ruleset,
    pub timeout: u32,
    pub map: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Ruleset {
    pub name: String,
    pub version: String,
    pub settings: RulesetSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct RulesetSettings {
    pub food_spawn_chance: u8,
    pub minimum_food: u8,
    pub hazard_damage_per_turn: u8,
    pub royale_settings: RoyaleSettings,
    pub squad_settings: SquadSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct RoyaleSettings {
    pub shrink_every_n_turns: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct SquadSettings {
    pub allow_body_collisions: bool,
    pub shared_elimination: bool,
    pub shared_health: bool,
    pub shared_length: bool,
}

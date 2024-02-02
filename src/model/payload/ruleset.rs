#[derive(Debug, serde::Deserialize)]
pub struct Ruleset {
    pub name: String,
    pub version: String,
    pub settings: RulesetSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct RulesetSettings {
    #[serde(rename = "foodSpawnChance")]
    pub food_spawn_chance: u8,
    #[serde(rename = "minimumFood")]
    pub minimum_food: u8,
    #[serde(rename = "hazardDamagePerTurn")]
    pub hazard_damage_per_turn: u8,
    #[serde(rename = "royale")]
    pub royale_settings: RoyaleSettings,
    #[serde(rename = "squad")]
    pub squad_settings: SquadSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct RoyaleSettings {
    #[serde(rename = "shrinkEveryNTurns")]
    pub shrink_every_n_turns: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct SquadSettings {
    #[serde(rename = "allowBodyCollisions")]
    pub allow_body_collisions: bool,
    #[serde(rename = "sharedElimination")]
    pub shared_elimination: bool,
    #[serde(rename = "sharedHealth")]
    pub shared_health: bool,
    #[serde(rename = "sharedLength")]
    pub shared_length: bool,
}

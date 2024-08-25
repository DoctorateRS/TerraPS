use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2CurrentPlayerProperty {
    pub exp: u16,
    pub level: u8,
    pub max_level: u8,
    pub hp: Rlv2CurrentPlayerPropertyHp,
    pub gold: u32,
    pub shield: u16,
    pub capacity: u16,
    pub population: Rlv2CurrentPlayerPropertyPopulation,
    pub con_perfect_battle: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Rlv2CurrentPlayerPropertyHp {
    pub current: u16,
    pub max: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Rlv2CurrentPlayerPropertyPopulation {
    pub cost: u16,
    pub max: u16,
}

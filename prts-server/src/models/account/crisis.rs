use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{models::EmptyMap, utils::time::time};

#[derive(Deserialize, Serialize)]
pub struct Crisis {
    lst: u64,
    nst: u64,
}

impl Default for Crisis {
    fn default() -> Self {
        let time = time();
        Self { lst: time, nst: time }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CrisisV2 {
    current: String,
    seasons: HashMap<String, ()>,
}

#[derive(Deserialize, Serialize)]
pub struct CrisisV2Season {
    permanent: String,
    temporary: HashMap<String, ()>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2SeasonPermanent {
    state: u8,
    score_total: Vec<u16>,
    score_single: Vec<u16>,
    comment: [u8; 0],
    rune: EmptyMap,
    ex_runes: HashMap<String, u8>,
    rune_pack: EmptyMap,
    challenge: EmptyMap,
    reward: EmptyMap,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2SeasonTemporary {
    state: u8,
    score_total: Vec<u16>,
    rune: EmptyMap,
    challenge: EmptyMap,
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    models::{EmptyMap, NullVec},
    utils::time::time,
};

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
    seasons: HashMap<String, CrisisV2Season>,
    shop: CrisisV2Shop,
}

#[derive(Deserialize, Serialize)]
pub struct CrisisV2Season {
    permanent: CrisisV2SeasonPermanent,
    temporary: HashMap<String, CrisisV2SeasonTemporary>,
    social: CrisisV2SeasonSocial,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2SeasonPermanent {
    state: u8,
    pub score_total: Vec<u16>,
    pub score_single: Vec<u16>,
    comment: NullVec,
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
    pub score_total: Vec<u16>,
    rune: EmptyMap,
    challenge: EmptyMap,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2SeasonSocial {
    assist_cnt: u8,
    max_pnt: i8,
    chars: NullVec,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2Shop {
    coin: u16,
    info: NullVec,
    progress_info: EmptyMap,
}

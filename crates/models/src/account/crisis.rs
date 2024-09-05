use std::collections::HashMap;

use common_utils::time;
use serde::{Deserialize, Serialize};

use crate::{NullObj, NullVec};

#[derive(Deserialize, Serialize)]
pub struct Crisis {
    pub lst: u64,
    pub nst: u64,
}

impl Default for Crisis {
    fn default() -> Self {
        let time = time(-1);
        Self { lst: time, nst: time }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct CrisisV2 {
    pub current: String,
    pub seasons: HashMap<String, CrisisV2Season>,
    pub shop: CrisisV2Shop,
}

#[derive(Deserialize, Serialize)]
pub struct CrisisV2Season {
    pub permanent: CrisisV2SeasonPermanent,
    pub temporary: HashMap<String, CrisisV2SeasonTemporary>,
    pub social: CrisisV2SeasonSocial,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2SeasonPermanent {
    pub state: u8,
    pub score_total: Vec<u16>,
    pub score_single: Vec<u16>,
    comment: NullVec,
    rune: HashMap<String, u8>,
    ex_runes: HashMap<String, u8>,
    rune_pack: HashMap<String, u8>,
    challenge: HashMap<String, u8>,
    reward: NullObj,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2SeasonTemporary {
    pub state: u8,
    pub score_total: Vec<u16>,
    rune: NullObj,
    challenge: NullObj,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2SeasonSocial {
    pub assist_cnt: u8,
    pub max_pnt: i8,
    chars: NullVec,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2Shop {
    pub coin: u16,
    info: NullVec,
    progress_info: NullObj,
}

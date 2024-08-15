use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::NullVec;

#[derive(Deserialize, Serialize)]
pub struct MedalData {
    id: String,
    val: NullVec,
    fts: u64,
    rts: u64,
}

impl MedalData {
    pub fn new(id: String) -> Self {
        Self { id, val: [], fts: 1695000000, rts: 1695000000 }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct Medal {
    medals: HashMap<String, MedalData>,
}

impl Medal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_medal(&mut self, id: String) {
        self.medals.insert(id.clone(), MedalData::new(id));
    }
}

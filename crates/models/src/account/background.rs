use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use common_utils::time;

#[derive(Deserialize, Serialize, Default)]
pub struct Background {
    pub selected: String,
    pub bgs: HashMap<String, Bg>,
}

impl Background {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_bg(&mut self, bgid: String) {
        self.selected = bgid;
    }

    pub fn add_bg(&mut self, bgid: String) {
        self.bgs.insert(bgid, Bg::default());
    }
}

#[derive(Deserialize, Serialize)]
pub struct Bg {
    pub unlock: u64,
}

impl Default for Bg {
    fn default() -> Self {
        Self { unlock: time(-1) }
    }
}

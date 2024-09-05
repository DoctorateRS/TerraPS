use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::NullVec;

#[derive(Deserialize, Serialize)]
pub struct Mainline {
    pub record: HashMap<String, u8>,
    pub cache: NullVec,
    pub version: u8,
}

impl Default for Mainline {
    fn default() -> Self {
        Self {
            record: Default::default(),
            cache: [],
            version: 1,
        }
    }
}

impl Mainline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_record(&mut self, key: String) {
        self.record.insert(key, 1);
    }
}

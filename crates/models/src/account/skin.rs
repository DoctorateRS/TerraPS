use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Skin {
    #[serde(rename = "characterSkins")]
    pub char_skins: HashMap<String, u8>,
}

impl Skin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_skin(&mut self, skin: String) {
        self.char_skins.insert(skin, 1);
    }
}

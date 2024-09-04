use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NameCardStyle {
    pub component_order: Vec<String>,
    pub skin: NameCardSkin,
}

impl NameCardStyle {
    pub fn new() -> Self {
        Self {
            component_order: Vec::new(),
            skin: NameCardSkin { selected: String::new(), state: HashMap::new() },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NameCardSkin {
    pub selected: String,
    pub state: HashMap<String, NameCardSkinState>,
}

#[derive(Serialize, Deserialize)]
pub struct NameCardSkinState {
    pub unlock: bool,
    pub progress: (),
}

impl Default for NameCardSkinState {
    fn default() -> Self {
        Self { unlock: true, progress: () }
    }
}

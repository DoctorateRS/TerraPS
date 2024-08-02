use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NameCardStyle {
    component_order: Vec<String>,
    skin: NameCardSkin,
}

#[derive(Serialize, Deserialize)]
struct NameCardSkin {
    selected: String,
    state: HashMap<String, NameCardSkinState>,
}

#[derive(Serialize, Deserialize)]
struct NameCardSkinState {
    unlock: bool,
    progress: (),
}

impl NameCardSkinState {
    pub const fn default() -> Self {
        Self { unlock: true, progress: () }
    }
}

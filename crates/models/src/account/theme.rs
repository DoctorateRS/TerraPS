use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use common_utils::time;

#[derive(Deserialize, Serialize, Default)]
pub struct HomeTheme {
    pub selected: String,
    pub themes: HashMap<String, Theme>,
}

impl HomeTheme {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_theme(&mut self, id: String) {
        self.selected = id
    }

    pub fn set_theme(&mut self, id: String) {
        self.themes.insert(id, Theme::default());
    }
}

#[derive(Deserialize, Serialize)]
pub struct Theme {
    unlock: u64,
}

impl Default for Theme {
    fn default() -> Self {
        Self { unlock: time(-1) }
    }
}

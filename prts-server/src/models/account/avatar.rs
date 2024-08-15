use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::time::time;

#[derive(Deserialize, Serialize)]
pub struct AvatarIcon {
    ts: u64,
    src: String,
}

impl AvatarIcon {
    pub fn new(is_init: bool) -> Self {
        Self {
            ts: time(),
            src: String::from(if is_init { "initial" } else { "other" }),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct Avatar {
    avatar_icon: HashMap<String, AvatarIcon>,
}

impl Avatar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_avatar(&mut self, id: String, is_init: bool) {
        self.avatar_icon.insert(id, AvatarIcon::new(is_init));
    }
}

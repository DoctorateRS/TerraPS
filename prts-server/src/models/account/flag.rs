use serde::{Deserialize, Serialize};

use crate::utils::time::time;

#[derive(Deserialize, Serialize)]
pub struct PushFlags {
    has_gifts: u8,
    has_friend_request: u8,
    has_clues: u8,
    has_free_level_gp: u8,
    status: u64,
}

impl Default for PushFlags {
    fn default() -> Self {
        Self {
            has_gifts: 0,
            has_clues: 0,
            has_free_level_gp: 0,
            has_friend_request: 0,
            status: time(),
        }
    }
}

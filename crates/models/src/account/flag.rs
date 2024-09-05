use serde::{Deserialize, Serialize};

use common_utils::time;

#[derive(Deserialize, Serialize)]
pub struct PushFlags {
    pub has_gifts: u8,
    pub has_friend_request: u8,
    pub has_clues: u8,
    pub has_free_level_gp: u8,
    pub status: u64,
}

impl Default for PushFlags {
    fn default() -> Self {
        Self {
            has_gifts: 0,
            has_clues: 0,
            has_free_level_gp: 0,
            has_friend_request: 0,
            status: time(-1),
        }
    }
}

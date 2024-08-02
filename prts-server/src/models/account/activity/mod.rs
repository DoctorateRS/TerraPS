use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::EmptyMap;

pub mod bossrush;

pub type VoidMap<K> = HashMap<K, EmptyMap>;

#[derive(Serialize, Deserialize)]
pub struct TypeAct24Side {
    act24side: Act24Side,
}

#[derive(Serialize, Deserialize)]
struct Act24Side {
    tool: Act24Tool,
}

#[derive(Serialize, Deserialize)]
struct Act24Tool {
    tool_trap: u8,
    tool_wirebug: u8,
    tool_bomb: u8,
    tool_flashbomb: u8,
}

impl TypeAct24Side {
    pub const fn default() -> Self {
        Self {
            act24side: Act24Side {
                tool: Act24Tool {
                    tool_trap: 1,
                    tool_wirebug: 1,
                    tool_bomb: 1,
                    tool_flashbomb: 1,
                },
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AprilFool {
    act5fun: AprilFoolData,
}

#[derive(Serialize, Deserialize)]
struct AprilFoolData {
    #[serde(rename = "isOpen")]
    is_open: bool,
}

impl AprilFool {
    pub const fn default() -> Self {
        Self { act5fun: AprilFoolData { is_open: true } }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DefaultObject {
    #[serde(rename = "1stact")]
    first_act: EmptyMap,
}

impl DefaultObject {
    pub const fn default() -> Self {
        Self { first_act: EmptyMap::new() }
    }
}

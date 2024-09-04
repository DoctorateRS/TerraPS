use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::NullObj;

mod bossrush;
pub use bossrush::*;

pub type MappingObject = HashMap<String, NullObj>;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityEnum {
    Bossrush(HashMap<String, Bossrush>),
    Act24Side(TypeAct24Side),
    AprilFool(AprilFool),
    Other(MappingObject),
}

#[derive(Serialize, Deserialize)]
pub struct TypeAct24Side {
    pub act24side: Act24Side,
}

#[derive(Serialize, Deserialize)]
pub struct Act24Side {
    pub tool: Act24Tool,
}

#[derive(Serialize, Deserialize)]
pub struct Act24Tool {
    pub tool_trap: u8,
    pub tool_wirebug: u8,
    pub tool_bomb: u8,
    pub tool_flashbomb: u8,
}

pub const ACT24SIDE: TypeAct24Side = TypeAct24Side {
    act24side: Act24Side {
        tool: Act24Tool {
            tool_trap: 1,
            tool_wirebug: 1,
            tool_bomb: 1,
            tool_flashbomb: 1,
        },
    },
};

#[derive(Serialize, Deserialize)]
pub struct AprilFool {
    pub act5fun: AprilFoolData,
}

#[derive(Serialize, Deserialize)]
pub struct AprilFoolData {
    #[serde(rename = "isOpen")]
    pub is_open: bool,
}

pub const APRILFOOL: AprilFool = AprilFool { act5fun: AprilFoolData { is_open: true } };

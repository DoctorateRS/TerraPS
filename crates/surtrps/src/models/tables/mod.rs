use std::path::Path;

use serde::Serialize;

use common_utils::read_json;
use serde_json::from_value;

pub mod activity;
pub mod character;
pub mod crisisv2;
pub mod stage;

macro_rules! impl_load {
    () => {};
}

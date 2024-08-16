use std::{collections::HashMap, default};

use serde::{Deserialize, Serialize};

mod addon;
mod chara;
mod squad;

pub use addon::*;
pub use squad::*;

use crate::utils::time::time;

#[derive(Deserialize, Serialize)]
pub struct CharGroupComponent {
    favor_point: u16,
}

impl Default for CharGroupComponent {
    fn default() -> Self {
        Self { favor_point: 25570 }
    }
}

use serde::{Deserialize, Serialize};

pub mod curprop;
pub mod current;
pub mod season;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2 {}

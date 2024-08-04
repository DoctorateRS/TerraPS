use crate::{models::PlayerDataDeltaStatic, utils::time::time};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AccountLogin {
    result: u8,
    uid: String,
    secret: &'static str,
    #[serde(rename = "serviceLicenseVersion")]
    service_license_version: u8,
}

impl AccountLogin {
    pub fn new(uid: String) -> Self {
        Self {
            result: 0,
            uid,
            secret: "yostar",
            service_license_version: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccountSyncStatus {
    ts: u64,
    result: HashMap<(), ()>,
    #[serde(rename = "playerDataDelta")]
    pdd: PlayerDataDeltaStatic,
}

impl AccountSyncStatus {
    pub fn default() -> Self {
        Self {
            ts: time(),
            result: HashMap::default(),
            pdd: PlayerDataDeltaStatic::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccountYostarAuthSubmit {
    result: u8,
    yostar_account: &'static str,
    yostar_token: &'static str,
    yostar_uid: &'static str,
}

impl AccountYostarAuthSubmit {
    pub const fn default() -> Self {
        Self {
            result: 0,
            yostar_uid: "terraps@rustlang.com",
            yostar_token: "a",
            yostar_account: "1",
        }
    }
}

use crate::{NullObj, PlayerDataDelta};
use common_utils::time;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AccountLogin {
    pub result: u8,
    pub uid: String,
    pub secret: &'static str,
    #[serde(rename = "serviceLicenseVersion")]
    pub service_license_version: u8,
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
    pub ts: u64,
    pub result: NullObj,
    #[serde(rename = "playerDataDelta")]
    pub pdd: PlayerDataDelta,
}

impl AccountSyncStatus {
    pub fn new() -> Self {
        Self {
            ts: time(-1),
            result: NullObj {},
            pdd: PlayerDataDelta::default(),
        }
    }
}

impl Default for AccountSyncStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccountYostarAuthSubmit<'a> {
    result: u8,
    yostar_account: &'a str,
    yostar_token: &'a str,
    yostar_uid: &'a str,
}

pub const ACC_YOSTAR_AUTH_SUB: AccountYostarAuthSubmit = AccountYostarAuthSubmit {
    result: 0,
    yostar_uid: "terraps@rustlang.com",
    yostar_token: "a",
    yostar_account: "1",
};

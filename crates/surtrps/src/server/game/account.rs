use std::fs::exists;

use axum::{http::HeaderMap, Json};
use common_utils::write_json;
use uuid::Uuid;

use anyhow::Result;

use models::{
    account::{
        sync::{AccountLogin, AccountSyncStatus},
        User,
    },
    tables::{CharacterTable, LoadTable, SkinTable},
    MiscResponse, BATCH_EVENT,
};

use crate::cnst::user::USER_JSON_PATH;

pub async fn login(header: HeaderMap) -> Json<AccountLogin> {
    let uid = header.get("Uid").and_then(|v| v.to_str().ok()).map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());

    Json(AccountLogin::new(uid))
}

pub async fn sync_data() {
    fn sync_data_inner() -> Result<()> {
        if !exists(USER_JSON_PATH)? {
            write_json(USER_JSON_PATH, User::default())?;
        }

        let skin_table = SkinTable::load()?;
        let char_table = CharacterTable::load()?;

        Ok(())
    }

    sync_data_inner().unwrap();
}

pub async fn sync_status() -> Json<AccountSyncStatus> {
    Json(AccountSyncStatus::default())
}

pub async fn sync_push_data<'a>() -> Json<MiscResponse<'a>> {
    Json(BATCH_EVENT)
}

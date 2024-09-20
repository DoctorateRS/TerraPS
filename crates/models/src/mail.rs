use std::{collections::HashMap, fs::File};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    #[serde(rename = "type")]
    pub t: String,
    pub count: u64,
}

impl Item {
    pub fn new<S: ToString>(id: S, t: S, count: u64) -> Self {
        Self {
            id: id.to_string(),
            t: t.to_string(),
            count,
        }
    }

    pub fn new_originium_shard(count: u64) -> Self {
        Self::new("4003", "DIAMOND_SHD", count)
    }

    pub fn new_headhunting_permit(count: u64) -> Self {
        Self::new("7003", "TKT_GACHA", count)
    }

    pub fn new_10x_headhunting_permit(count: u64) -> Self {
        Self::new("7004", "TKT_GACHA_10", count)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mail {
    pub from: String,
    pub subject: String,
    pub content: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailManager {
    pub received_ids: Vec<u32>,
    pub deleted_ids: Vec<u32>,
    pub mail_list: HashMap<String, Mail>,
}

impl MailManager {
    pub fn load() -> Result<Self> {
        Ok(from_reader(File::open("../../config/mails.json")?)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailConfig {
    pub create_at: u64,
    pub has_item: u8,
    pub mail_id: String,
    pub state: u8,
    #[serde(rename = "type")]
    pub t: u8,
}

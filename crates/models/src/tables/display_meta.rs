use std::{collections::HashMap, fs::File};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayMetaTable {
    pub player_avatar_data: PlayerAvatarData,
    pub home_background_data: HomeBackgroundData,
    pub name_card_v2_data: NamecardV2Data,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAvatarData {
    pub avatar_list: Vec<AvatarData>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarData {
    pub avatar_id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeBackgroundData {
    pub home_bg_data_list: Vec<BackgroundData>,
    pub theme_list: Vec<ThemeData>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundData {
    pub bg_id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeData {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamecardV2Data {
    pub skin_data: HashMap<String, NamecardV2SkinData>,
}

#[derive(Deserialize, Serialize)]
pub struct NamecardV2SkinData {
    pub id: String,
}

impl LoadTable for DisplayMetaTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/display_meta_table.json")?)?)
    }
}

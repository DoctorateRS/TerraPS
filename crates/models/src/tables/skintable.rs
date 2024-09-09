use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use super::LoadTable;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RootSkinTableObject {
    pub char_skins: HashMap<String, CharSkinTable>,
}

impl LoadTable for RootSkinTableObject {
    type Err = Error;

    fn load() -> Result<Self> {
        Ok(from_reader(File::open("../../data/excel/skin_table.json")?)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharSkinTable {
    pub skin_id: String,
    pub char_id: String,
    pub token_skin_map: Option<Value>,
    pub illust_id: Option<String>,
    pub dyn_illust_id: Option<String>,
    pub avatar_id: Option<String>,
    pub portrait_id: Option<String>,
    pub dyn_portrait_id: Option<String>,
    pub dyn_entrance_id: Option<String>,
    pub building_id: Option<String>,
    pub battle_skin: BattleSkinTable,
    pub is_buy_skin: bool,
    pub tmpl_id: Option<String>,
    pub voice_id: Option<String>,
    pub voice_type: String,
    pub display_skin: DisplaySkinTable,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BattleSkinTable {
    pub overwrite_prefab: bool,
    pub skin_or_prefab_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DisplaySkinTable {
    pub skin_name: Option<String>,
    pub color_list: Option<Vec<String>>,
    pub title_list: Option<Vec<String>>,
    pub model_name: Option<String>,
    pub drawer_list: Option<Vec<String>>,
    pub designer_list: Option<Vec<String>>,
    pub skin_group_id: Option<String>,
    pub skin_group_name: Option<String>,
    pub skin_group_sort_index: i32,
    pub content: Option<String>,
    pub dialog: Option<String>,
    pub usage: Option<String>,
    pub description: Option<String>,
    pub obtain_approach: Option<String>,
    pub sort_id: i32,
    pub display_tag_id: Option<String>,
    pub get_time: u64,
    pub on_year: u32,
    pub on_period: u32,
}
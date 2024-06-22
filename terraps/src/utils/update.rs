use std::str::{from_utf8, FromStr};

use tokio::spawn;

use crate::{
    constants::{config::CONFIG_JSON_PATH, global_url::*, url::*, user::USER_GACHA_PATH},
    utils::json::get_keys,
};

use super::{crypto::base64::decrypt, game::update_data};

use anyhow::Result;
use common_utils::{read_json, write_json};
use reqwest::get;
use serde_json::{from_str, json, Value};

pub enum Mode {
    Cn,
    Global,
}

impl FromStr for Mode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cn" => Ok(Mode::Cn),
            "global" => Ok(Mode::Global),
            _ => Err(()),
        }
    }
}

const VER_CONF: &str = "aHR0cHM6Ly9hay1jb25mLmh5cGVyZ3J5cGguY29tL2NvbmZpZy9wcm9kL29mZmljaWFsL0FuZHJvaWQvdmVyc2lvbg==";
const NW_CONF: &str = "aHR0cHM6Ly9hay1jb25mLmh5cGVyZ3J5cGguY29tL2NvbmZpZy9wcm9kL29mZmljaWFsL25ldHdvcmtfY29uZmln";

pub async fn update_cn_config() -> Result<bool> {
    let mut excel_update = false;
    let stcf = read_json(CONFIG_JSON_PATH);
    let mut config = stcf.clone();

    let old_res_version = &stcf["version"]["android"]["resVersion"];
    let old_client_version = &stcf["version"]["android"]["clientVersion"];
    let old_func_ver = &stcf["networkConfig"]["cn"]["content"]["funcVer"];

    let new_ver_config = get(from_utf8(decrypt(VER_CONF)?.as_slice())?).await?.json::<Value>().await?;
    let new_nw_config = get(from_utf8(decrypt(NW_CONF)?.as_slice())?).await?.json::<Value>().await?;

    if old_res_version != &new_ver_config["resVersion"] {
        excel_update = true;
        config["version"]["android"]["resVersion"] = json!(new_ver_config["resVersion"]);
    }
    if old_client_version != &new_ver_config["clientVersion"] {
        excel_update = true;
        config["version"]["android"]["clientVersion"] = json!(new_ver_config["clientVersion"]);
    }

    let content = from_str::<Value>(new_nw_config["content"].as_str().unwrap())?;
    let func_ver = &content["funcVer"];
    if old_func_ver != func_ver {
        println!("Update detected. Updating config...");

        excel_update = true;
        config["networkConfig"]["cn"]["content"]["funcVer"] = json!(func_ver);
        config["networkConfig"]["cn"]["content"]["configs"][func_ver.as_str().unwrap()] =
            config["networkConfig"]["cn"]["content"]["configs"][old_func_ver.as_str().unwrap()].clone();
        config["networkConfig"]["cn"]["content"]["configs"]
            .as_object_mut()
            .unwrap()
            .remove(old_func_ver.as_str().unwrap());
    }

    write_json(CONFIG_JSON_PATH, config);

    Ok(excel_update)
}

pub async fn excel_update(mode: Mode) -> Result<()> {
    let list: Vec<&'static str> = match mode {
        Mode::Cn => vec![
            ACTIVITY_TABLE_URL,
            CHARM_TABLE_URL,
            SKIN_TABLE_URL,
            CHARACTER_TABLE_URL,
            BATTLEEQUIP_TABLE_URL,
            EQUIP_TABLE_URL,
            STORY_TABLE_URL,
            STAGE_TABLE_URL,
            RL_TABLE_URL,
            DM_TABLE_URL,
            RETRO_TABLE_URL,
            HANDBOOK_INFO_TABLE_URL,
            TOWER_TABLE_URL,
            BUILDING_TABLE_URL,
            SANDBOX_TABLE_URL,
            STORY_REVIEW_TABLE_URL,
            STORY_REVIEW_META_TABLE_URL,
            ENEMY_HANDBOOK_TABLE_URL,
            MEDAL_TABLE_URL,
            CHARWORD_TABLE_URL,
            GACHA_TABLE_URL,
            GAMEDATA_CONST_URL,
        ],
        Mode::Global => vec![
            GLOBAL_ACTIVITY_TABLE_URL,
            GLOBAL_CHARM_TABLE_URL,
            GLOBAL_SKIN_TABLE_URL,
            GLOBAL_CHARACTER_TABLE_URL,
            GLOBAL_BATTLEEQUIP_TABLE_URL,
            GLOBAL_EQUIP_TABLE_URL,
            GLOBAL_STORY_TABLE_URL,
            GLOBAL_STAGE_TABLE_URL,
            GLOBAL_RL_TABLE_URL,
            GLOBAL_DM_TABLE_URL,
            GLOBAL_RETRO_TABLE_URL,
            GLOBAL_HANDBOOK_INFO_TABLE_URL,
            GLOBAL_TOWER_TABLE_URL,
            GLOBAL_BUILDING_TABLE_URL,
            GLOBAL_SANDBOX_TABLE_URL,
            GLOBAL_STORY_REVIEW_TABLE_URL,
            GLOBAL_STORY_REVIEW_META_TABLE_URL,
            GLOBAL_ENEMY_HANDBOOK_TABLE_URL,
            GLOBAL_MEDAL_TABLE_URL,
            GLOBAL_CHARWORD_TABLE_URL,
            GLOBAL_GACHA_TABLE_URL,
            GLOBAL_GAMEDATA_CONST_URL,
        ],
    };

    let handles = list.iter().map(|link| spawn(update_excel_data(link))).collect::<Vec<_>>();
    for handle in handles {
        handle.await??;
    }
    Ok(())
}

async fn update_excel_data(link: &str) -> Result<()> {
    let path = link
        .replace(
            "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata",
            "./data",
        )
        .replace(
            "https://ak-conf.hypergryph.com/config/prod/announce_meta/Android",
            "./data/announce",
        );
    let json = get(link).await?.json::<Value>().await?;
    write_json(&path, json);
    println!("Updated: {}", path.replace("./data/announce", "").replace("./data", ""));
    Ok(())
}

pub async fn update_gacha() -> Result<()> {
    const WELFARE_CHAR_LIST: [&str; 6] = [
        "char_474_glady",
        "char_4042_lumen",
        "char_427_vigil",
        "char_1031_slent2",
        "char_4011_lessng",
        "char_4134_cetsyr",
    ];
    let mut gacha = read_json(USER_GACHA_PATH);
    let char_table = update_data(CHARACTER_TABLE_URL).await;
    let gacha_advanced = gacha["advanced"].clone();
    let gacha_preloaded = gacha_advanced
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x["charId"].as_str().unwrap())
        .collect::<Vec<&str>>();
    for char in get_keys(&char_table) {
        if char_table[&char]["rarity"].as_str().unwrap() == "TIER_6"
            && !WELFARE_CHAR_LIST.contains(&char.as_str())
            && char.starts_with("char_")
            && !gacha_preloaded.contains(&char.as_str())
        {
            gacha["advanced"].as_array_mut().unwrap().push(json!({
                "charId": char,
                "isNew": 1
            }));
        }
    }
    write_json(USER_GACHA_PATH, &gacha);
    Ok(())
}

use crate::{
    constants::{config::CONFIG_JSON_PATH, url::*, user::USER_GACHA_PATH},
    utils::json::get_keys,
};

use super::{
    game::update_data,
    json::{read_json, write_json},
};

use anyhow::Result;
use reqwest::get;
use serde_json::{from_str, json, Value};

const VER_AK_CONF: &str = "https://ak-conf.hypergryph.com/config/prod/official/Android/version";
const NW_AK_CONF: &str = "https://ak-conf.hypergryph.com/config/prod/official/network_config";

pub async fn update_config() -> Result<bool> {
    let mut excel_update = false;
    let stcf = read_json(CONFIG_JSON_PATH);
    let mut config = stcf.clone();

    let old_res_version = &stcf["version"]["android"]["resVersion"];
    let old_client_version = &stcf["version"]["android"]["clientVersion"];
    let old_func_ver = &stcf["networkConfig"]["cn"]["content"]["funcVer"];

    let new_ver_config = get(VER_AK_CONF).await?.json::<Value>().await?;
    let new_nw_config = get(NW_AK_CONF).await?.json::<Value>().await?;

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

pub async fn excel_update() -> Result<()> {
    let list = vec![
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
    ];

    for url in list {
        let path = url
            .replace(
                "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata",
                "./data",
            )
            .replace(
                "https://ak-conf.hypergryph.com/config/prod/announce_meta/Android",
                "./data/announce",
            );
        let json = get(url).await?.json::<Value>().await?;
        write_json(&path, json);
    }
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

    println!("Generating Gacha...");
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

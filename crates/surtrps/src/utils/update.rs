use std::{
    collections::HashMap,
    fs::File,
    io::read_to_string,
    str::{from_utf8, FromStr},
};

use anyhow::{anyhow, Result};
use common_utils::write_json;
use reqwest::get;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use tokio::spawn;

use crate::{
    cnst::{
        config::{NETWORK_CONFIG_TEMPLATE_PATH, VERSION_CONFIG_PATH},
        global_url::*,
        url::*,
    },
    models::prod::{
        network::{NetworkConfigContent, ProdAndroidNetworkConfig},
        ProdAndroidNetwork, ProdAndroidVersion,
    },
    SERVER_CONFIG,
};

use super::b64::decrypt;

#[derive(Clone, Copy)]
pub enum Mode {
    Cn,
    Global,
}

impl Mode {
    fn to_str(self) -> &'static str {
        match self {
            Mode::Cn => "cn",
            Mode::Global => "global",
        }
    }
}

#[derive(Deserialize, Serialize)]
struct AndroidVersionCfgWrapper {
    pub android: ProdAndroidVersion,
}

impl FromStr for Mode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cn" | "china" => Ok(Mode::Cn),
            "gb" | "global" => Ok(Mode::Global),
            _ => Err(()),
        }
    }
}

const VER_CONF: &str = "aHR0cHM6Ly9hay1jb25mLmh5cGVyZ3J5cGguY29tL2NvbmZpZy9wcm9kL29mZmljaWFsL0FuZHJvaWQvdmVyc2lvbg==";
const NW_CONF: &str = "aHR0cHM6Ly9hay1jb25mLmh5cGVyZ3J5cGguY29tL2NvbmZpZy9wcm9kL29mZmljaWFsL25ldHdvcmtfY29uZmln";

pub async fn update() -> Result<()> {
    let mut excel_update_required = false;

    let mode = Mode::from_str(&SERVER_CONFIG.mode).map_err(|_| anyhow!("Failed to parse mode string."))?;
    let mut old_ver_cfgs = {
        let content = read_to_string(File::open(VERSION_CONFIG_PATH)?)?;

        from_str::<HashMap<String, AndroidVersionCfgWrapper>>(&content)?
    };

    let (mut old_ver_cfg, tmp) = match mode {
        Mode::Cn => (old_ver_cfgs.remove("version").unwrap(), old_ver_cfgs.remove("versionGlobal").unwrap()),
        Mode::Global => (old_ver_cfgs.remove("versionGlobal").unwrap(), old_ver_cfgs.remove("version").unwrap()),
    };

    let new_ver_cfg = get(from_utf8(&decrypt(VER_CONF)?)?).await?.json::<ProdAndroidVersion>().await?;

    if old_ver_cfg.android.res_version != new_ver_cfg.res_version {
        excel_update_required = true;
        old_ver_cfg.android.res_version = new_ver_cfg.res_version;
    }

    if old_ver_cfg.android.client_version != new_ver_cfg.client_version {
        excel_update_required = true;
        old_ver_cfg.android.client_version = new_ver_cfg.client_version;
    }

    match mode {
        Mode::Cn => {
            old_ver_cfgs.insert("version".into(), old_ver_cfg);
            old_ver_cfgs.insert("versionGlobal".into(), tmp);
        }
        Mode::Global => {
            old_ver_cfgs.insert("versionGlobal".into(), old_ver_cfg);
            old_ver_cfgs.insert("version".into(), tmp);
        }
    };

    write_json(VERSION_CONFIG_PATH, old_ver_cfgs)?;

    let new_net_cfg = get(from_utf8(decrypt(NW_CONF)?.as_slice())?).await?.json::<ProdAndroidNetwork>().await?;

    let new_nwcfg_content = from_str::<NetworkConfigContent>(&new_net_cfg.content)?;

    let mut old_net_cfgs = from_str::<HashMap<String, ProdAndroidNetworkConfig>>(&read_to_string(File::open(NETWORK_CONFIG_TEMPLATE_PATH)?)?)?;

    let mut old_net_cfg = old_net_cfgs.remove(mode.to_str()).unwrap();

    if old_net_cfg.content.config_ver != new_nwcfg_content.config_ver {
        let mut tmp_nwcfg = new_nwcfg_content.configs.get(&new_nwcfg_content.config_ver).unwrap().clone();

        tmp_nwcfg.network.pkg_ad = None;
        tmp_nwcfg.network.pkg_ios = None;

        old_net_cfg.content.config_ver = new_nwcfg_content.config_ver.clone();
        old_net_cfg.content.configs.insert(new_nwcfg_content.config_ver, tmp_nwcfg);
    }

    old_net_cfgs.insert(mode.to_str().to_string(), old_net_cfg);
    write_json(NETWORK_CONFIG_TEMPLATE_PATH, old_net_cfgs)?;

    if excel_update_required {
        update_excel(mode).await?
    }

    Ok(())
}

async fn update_excel(mode: Mode) -> Result<()> {
    let update_list = match mode {
        Mode::Cn => [
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
        Mode::Global => [
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

    let handles = update_list.iter().map(|&link| spawn(update_excel_data(link))).collect::<Vec<_>>();

    for handle in handles {
        handle.await??
    }

    println!("Excel data update succeed.");

    Ok(())
}

async fn update_excel_data(link: &str) -> Result<()> {
    let path = link
        .replace("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata", "./data")
        .replace("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android", "./data/announce")
        .replace("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData_YoStar/main/en_US/gamedata/", "./dataGB/")
        .replace("| GLOBAL AK ANNOUNCE |", "./dataGB/announce");
    let json = get(link).await?.json::<Value>().await?;
    write_json(&path, json).unwrap_or(());
    println!("Updated: {}", path.replace("./data/announce", "").replace("./data", ""));
    Ok(())
}

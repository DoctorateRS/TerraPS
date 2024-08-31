use std::{
    collections::HashMap,
    fs::File,
    io::read_to_string,
    str::{from_utf8, FromStr},
};

use anyhow::{anyhow, Result};
use reqwest::get;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{
    cnst::config::VERSION_CONFIG_PATH,
    models::prod::{ProdAndroidNetwork, ProdAndroidVersion},
    SERVER_CONFIG,
};

use super::b64::decrypt;

pub enum Mode {
    Cn,
    Global,
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

pub async fn update_config() -> Result<()> {
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

    let new_ver_cfg = get(from_utf8(decrypt(VER_CONF)?.as_slice())?).await?.json::<ProdAndroidVersion>().await?;

    if old_ver_cfg.android.res_version != new_ver_cfg.res_version {
        excel_update_required = true;
        old_ver_cfg.android.res_version = new_ver_cfg.res_version;
    }

    if old_ver_cfg.android.client_version != new_ver_cfg.client_version {
        excel_update_required = true;
        old_ver_cfg.android.client_version = new_ver_cfg.client_version;
    }

    let new_net_cfg = get(from_utf8(decrypt(NW_CONF)?.as_slice())?).await?.json::<ProdAndroidNetwork>().await?;

    Ok(())
}

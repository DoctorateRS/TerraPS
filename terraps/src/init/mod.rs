use std::{fs::write, path::Path};

use anyhow::Result;
use file::mkdir;

use crate::constants::config::{ASSIST_JSON_PATH, CONFIG_JSON_PATH, MAILLIST_PATH};

mod file;

const DATA_DIRS: [&str; 9] = [
    "announce", "crisis", "crisisv2", "excel", "gacha", "rlv2", "sandbox", "tower", "user",
];

pub fn init() -> Result<()> {
    if !Path::new("./config/").exists() {
        mkdir("./config/")?;
        let cfg = include_str!("../../../config/config.json");
        write(CONFIG_JSON_PATH, cfg)?;
        let assists = include_str!("../../../config/assist.json");
        write(ASSIST_JSON_PATH, assists)?;
        let mails = include_str!("../../../config/mails.json");
        write(MAILLIST_PATH, mails)?;
        let multi_user_cfg = include_str!("../../../config/multiUserConfig.json");
        write("./config/multiUserConfig.json", multi_user_cfg)?;
    }
    if !Path::new("./data/").exists() {
        mkdir("./data/")?;
        for dir in DATA_DIRS.iter() {
            mkdir(format!("./data/{}", dir))?;
        }
    }
    Ok(())
}

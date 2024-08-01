use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::write,
    path::Path,
};

use anyhow::Result;
use file::mkdir;

use crate::{
    constants::{
        config::{ASSIST_JSON_PATH, CONFIG_JSON_PATH, MAILLIST_PATH, RLV2_CONFIG_PATH, SQUADS_PATH, SYNC_DATA_TEMPLATE_PATH},
        templates::SANDBOX_TEMPLATE,
        user::BATTLE_REPLAY_JSON_PATH,
    },
    utils::update::{excel_update, Mode},
};

mod file;

const DATA_DIRS: [&str; 9] = ["announce", "crisis", "crisisv2", "excel", "gacha", "rlv2", "sandbox", "tower", "user"];

pub async fn init() -> Result<()> {
    init_cfg()?;
    init_data().await?;
    Ok(())
}

fn init_cfg() -> Result<()> {
    if !Path::new("./config/").exists() {
        mkdir("./config/")?;
    } else {
        return Ok(());
    }
    let cfg = include_str!("../../../config/config.json");
    write(CONFIG_JSON_PATH, cfg)?;
    let assists = include_str!("../../../config/assist.json");
    write(ASSIST_JSON_PATH, assists)?;
    let mails = include_str!("../../../config/mails.json");
    write(MAILLIST_PATH, mails)?;
    let multi_user_cfg = include_str!("../../../config/multiUserConfig.json");
    write("./config/multiUserConfig.json", multi_user_cfg)?;
    let rlv2_cfg = include_str!("../../../config/rlv2Config.json");
    write(RLV2_CONFIG_PATH, rlv2_cfg)?;
    let squads = include_str!("../../../config/squads.json");
    write(SQUADS_PATH, squads)?;
    let sync_data_tmpl = include_str!("../../../config/syncData.json");
    write(SYNC_DATA_TEMPLATE_PATH, sync_data_tmpl)?;
    Ok(())
}

async fn init_data() -> Result<()> {
    if !Path::new("./data/").exists() {
        mkdir("./data/")?;
        for dir in DATA_DIRS.iter() {
            mkdir(format!("./data/{}", dir))?;
        }
        let announce = include_str!("../../../data/announce/announcement.meta.json");
        write("./data/announce/announcement.meta.json", announce)?;
        let preannounce = include_str!("../../../data/announce/preannouncement.meta.json");
        write("./data/announce/preannouncement.meta.json", preannounce)?;

        let cc1 = include_str!("../../../data/crisisv2/cc1.json");
        write("./data/crisisv2/cc1.json", cc1)?;
        let cc2 = include_str!("../../../data/crisisv2/cc2.json");
        write("./data/crisisv2/cc2.json", cc2)?;

        let awaiter = async {
            excel_update(Mode::Cn).await.map_err(|_| ExcelUpdateErr)?;
            Ok::<(), ExcelUpdateErr>(())
        };
        awaiter.await?;

        let sandbox_tmpl = include_str!("../../../data/sandbox/sandbox.json");
        write(SANDBOX_TEMPLATE, sandbox_tmpl)?;

        let battlereplays = include_str!("../../../data/user/battleReplays.json");
        write(BATTLE_REPLAY_JSON_PATH, battlereplays)?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct ExcelUpdateErr;

impl Display for ExcelUpdateErr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Failed to update excel data.")
    }
}

impl Error for ExcelUpdateErr {}
unsafe impl Send for ExcelUpdateErr {}
unsafe impl Sync for ExcelUpdateErr {}

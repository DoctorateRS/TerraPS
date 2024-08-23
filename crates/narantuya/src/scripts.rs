use anyhow::Result;
use common_utils::{ServerConfig, UserConfig};
use std::{fs::File, io::Read};

pub fn get_script() -> Result<String> {
    let mut script = File::open("./scripts/mumu6/_.js")?;
    let mut sc_buf = String::new();
    let server = ServerConfig::load()?;
    let usr_conf = UserConfig::load()?;
    script.read_to_string(&mut sc_buf)?;
    let sc_buf = if server.no_proxy {
        sc_buf.replace("@@@DOCTORATE_HOST@@@", "NO_PROXY")
    } else {
        sc_buf.replace("@@@DOCTORATE_HOST@@@", &server.host)
    };
    let sc_buf = sc_buf
        .replace("@@@DOCTORATE_PORT@@@", &server.port.to_string())
        .replace("@@@DOCTORATE_ACTIVITY_MIN_START_TS@@@", &usr_conf.act_min_start_ts.to_string())
        .replace("@@@DOCTORATE_ACTIVITY_MAX_START_TS@@@", &usr_conf.act_max_start_ts.to_string());
    Ok(sc_buf)
}

pub fn get_vision() -> Result<String> {
    let usr_conf = UserConfig::load()?;
    if usr_conf.vision {
        let mut vision = File::open("./scripts/mumu6/vision.js")?;
        let mut vi_buf = String::new();
        vision.read_to_string(&mut vi_buf)?;
        Ok(vi_buf)
    } else {
        Ok(String::new())
    }
}

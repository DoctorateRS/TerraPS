use anyhow::Result;
use common_utils::read_json;
use frida::{DeviceManager, Frida, ScriptOption, ScriptRuntime, Session, SpawnOptions};
use std::{fs::File, io::Read, process::Command};

mod b64;
mod init;
use crate::b64::B64Decoder;

const GLOBAL: &str = "Y29tLllvU3RhckVOLkFya25pZ2h0cw==";
const CN: &str = "Y29tLmh5cGVyZ3J5cGguYXJrbmlnaHRz";

fn main() -> Result<()> {
    let config = read_json("./config/config.json");

    let mut cmd_terminal = Command::new("cmd");

    let cmds = vec![
        "/C adb.exe root",
        "/C adb.exe connect 127.0.0.1:7555",
        "/C adb.exe reverse tcp:8443 tcp:8443",
        "/C adb.exe shell /data/local/tmp/frida-server &",
    ];

    for cmd in cmds {
        cmd_terminal.args([cmd]);
    }

    let frida = unsafe { Frida::obtain() };
    let device_manager = DeviceManager::obtain(&frida);
    let mut device = device_manager
        .enumerate_all_devices()
        .into_iter()
        .find(|device| device.get_id() == "127.0.0.1:7555")
        .unwrap();

    let def = SpawnOptions::default();

    let game = if config["server"]["mode"].as_str().unwrap() == "cn" {
        B64Decoder::new(CN).decode()?
    } else {
        B64Decoder::new(GLOBAL).decode()?
    };

    let game_pid = device.spawn(game, &def)?;
    let session = device.attach(game_pid)?;

    inject_script(&session)?;

    Ok(())
}

fn inject_script(session: &Session) -> Result<()> {
    let mut script_option = ScriptOption::new()
        .set_runtime(ScriptRuntime::Default)
        .set_name("_.js");
    let mut vision_option = ScriptOption::new()
        .set_runtime(ScriptRuntime::Default)
        .set_name("vision.js");
    let mut script = File::open("./scripts/_.js")?;
    let mut sc_buf = String::new();
    script.read_to_string(&mut sc_buf)?;
    let mut vision = File::open("./scripts/vision.js")?;
    let mut vi_buf = String::new();
    vision.read_to_string(&mut vi_buf)?;
    session.create_script(&sc_buf, &mut script_option)?;
    session.create_script(&vi_buf, &mut vision_option)?;
    Ok(())
}

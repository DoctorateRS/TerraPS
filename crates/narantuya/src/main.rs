use adb::Os;
use anyhow::Result;
use common_utils::{decrypt, ServerConfig};
use frida::{DeviceManager, Frida, ScriptOption, ScriptRuntime, Session, SpawnOptions};
use scripts::{get_script, get_vision};
use std::process::Command;

mod adb;
mod config;
mod scripts;

const GLOBAL: &str = "Y29tLllvU3RhckVOLkFya25pZ2h0cw==";
const CN: &str = "Y29tLmh5cGVyZ3J5cGguYXJrbmlnaHRz";

#[tokio::main]
async fn main() -> Result<()> {
    let os = Os::new();
    os.install_adb().await?;

    let server_conf = ServerConfig::load()?;

    let mut cmd_terminal = Command::new("cmd");

    let cmds = vec![
        "/C ./platform-tools/adb.exe root",
        "/C ./platform-tools/adb.exe connect 127.0.0.1:7555",
        "/C ./platform-tools/adb.exe reverse tcp:8443 tcp:8443",
        "/C ./platform-tools/adb.exe shell /data/local/tmp/frida-server &",
    ];

    for cmd in cmds {
        cmd_terminal.args([cmd]);
    }

    let frida = unsafe { Frida::obtain() };
    let device_manager = DeviceManager::obtain(&frida);
    let mut device = device_manager.enumerate_all_devices().into_iter().find(|device| device.get_id() == "127.0.0.1:7555").unwrap();

    let def = SpawnOptions::default();

    let game = String::from_utf8(if &server_conf.mode == "cn" { decrypt(CN)? } else { decrypt(GLOBAL)? })?;

    let game_pid = device.spawn(game, &def)?;
    let session = device.attach(game_pid)?;

    inject_script(&session)?;

    Ok(())
}

fn inject_script(session: &Session) -> Result<()> {
    let mut script_option = ScriptOption::new().set_runtime(ScriptRuntime::Default).set_name("_.js");
    let mut vision_option = ScriptOption::new().set_runtime(ScriptRuntime::Default).set_name("vision.js");
    let script = get_script()?;
    let vision = get_vision()?;
    session.create_script(&script, &mut script_option)?;
    session.create_script(&vision, &mut vision_option)?;
    Ok(())
}

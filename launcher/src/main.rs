use anyhow::Result;
use common_utils::read_json;
use frida::{DeviceManager, Frida, SpawnOptions};
use std::process::Command;

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
    device.spawn(game, &def)?;
    Ok(())
}

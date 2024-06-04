use anyhow::Result;
use common_utils::read_json;
use frida::{DeviceManager, Frida, SpawnOptions};
use std::process::Command;

const GLOBAL: &str = "global";
const CN: &str = "cn";

fn main() -> Result<()> {
    let config = read_json("./config/config.json");

    let mut cmd_terminal = Command::new("cmd");

    let cmds = vec![
        "/C adb root",
        "/C adb connect 127.0.0.1:7555",
        "/C adb reverse tcp:8443 tcp:8443",
        "/C adb shell /data/local/tmp/frida-server &",
    ];

    for cmd in cmds {
        let output = cmd_terminal.args([cmd]);
        println!("Result: {:?}", output);
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
        CN
    } else {
        GLOBAL
    };
    match device.spawn(game, &def) {
        Ok(result) => {
            println!("PID: {}", result);
            device.resume(result)?;
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

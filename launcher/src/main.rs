use frida::{DeviceManager, Frida, SpawnOptions};
use std::process::Command;

fn main() {
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
    match device.spawn("com.hypergryph.arknights", &SpawnOptions::default()) {
        Ok(result) => {
            println!("PID: {}", result);
            device.resume(result).unwrap();
        }
        Err(e) => println!("Error: {}", e),
    }
}

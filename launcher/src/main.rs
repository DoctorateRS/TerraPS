// Not fully functional yet, frida crate is also giving me errors out of nowhere, need to figure out what's wrong

fn main() {
    let mut cmd_terminal = std::process::Command::new("cmd");
    let output = cmd_terminal.args(["/C adb root"]);
    println!("Result: {:?}", output);
    let output = cmd_terminal.args(["/C adb connect 127.0.0.1:7555"]);
    println!("Result: {:?}", output);
    let output = cmd_terminal.args(["/C adb reverse tcp:8443 tcp:8443"]);
    println!("Result: {:?}", output);
    let output = cmd_terminal.args(["/C adb shell /data/local/tmp/frida-server &"]);
    println!("Result: {:?}", output);

    let frida = unsafe { frida::Frida::obtain() };
    let device_manager = frida::DeviceManager::obtain(&frida);
    let mut device = device_manager.enumerate_all_devices().into_iter().find(|device| device.get_id() == "127.0.0.1:7555").unwrap();
    match device.spawn(
        "com.hypergryph.arknights",
        &frida::SpawnOptions::default(),
    ) {
        Ok(result) => {
            println!("PID: {}", result);
            device.resume(result).unwrap();
    },
        Err(e) => println!("Error: {}", e),
    }
}
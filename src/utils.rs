use reqwest::get;
use serde::Serialize;
use serde_json::{from_reader, ser::PrettyFormatter, to_writer_pretty, Serializer, Value};
use std::{fs::File, io::BufReader};

pub async fn update_data(url: &str) -> Value {
    const BASE_URL_LIST: [(&str, &str); 2] = [
        (
            "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata",
            "./data",
        ),
        // ("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/en_US/gamedata", "./data"),
        (
            "https://ak-conf.hypergryph.com/config/prod/announce_meta/Android",
            "./data/announce",
        ),
        // ("https://ark-us-static-online.yo-star.com/announce/Android", "./data/announce"),
    ];

    let mut local_path = String::new();

    for path_pair in BASE_URL_LIST {
        local_path = url.replace(path_pair.0, path_pair.1);
    }

    if url.contains("Android/version") {
        match get(url).await.unwrap().json::<Value>().await {
            Ok(value) => value,
            Err(_) => panic!("Unable to process request."),
        }
    } else {
        read_json(&local_path)
    }
}

pub fn read_json(path: &str) -> Value {
    let json_reader = BufReader::new(match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Path {} not found.", path),
    });
    match from_reader(json_reader) {
        Ok(value) => value,
        Err(_) => panic!("Unable to read JSON."),
    }
}

pub fn write_json(path: &str, value: Value) {
    let file = File::create(path).unwrap();
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser).unwrap();
    match to_writer_pretty(file, &value) {
        Ok(_) => (),
        Err(_) => panic!("Unable to write JSON."),
    }
}

pub fn _decrypt_battle_data(_data: &str, _login_time: u64) -> Value {
    todo!("decrypt_battle_data")
}

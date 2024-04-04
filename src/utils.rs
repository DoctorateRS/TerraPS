use reqwest::get;
use serde::Serialize;
use serde_json::{from_reader, ser::PrettyFormatter, to_writer_pretty, Serializer, Value};
use std::{fs::File, io::BufReader, num::ParseIntError};


pub async fn update_data(url: &str) -> Value {
    let local_path = url
        .replace(
            "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata",
            "./data",
        )
        .replace(
            "https://ak-conf.hypergryph.com/config/prod/announce_meta/Android",
            "./data/announce",
        );

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

pub fn decrypt_battle_data(data: &str, login_time: Option<u64>)  {

    const LOG_TOKEN_KEY: &str = "pM6Umv*^hVQuB6t&";

    let login_time = match login_time {
        Ok(time) => time,
        None => 1672502400,
    };
    let len = &data.len();
    let data = data[..len];
    let src = LOG_TOKEN_KEY.to_string() + login_time.as_str();

    let battle_data = match from_hex(data) {
        Ok(data) => match String::from_utf8(data) {
            Ok(data) => data,
            Err(e) => panic!("Error parsing UTF-8: {e}")
        },
        Err(e) => panic!("Error parsing Integer:{}",e)
    };


}

pub fn get_keys(value: &Value) -> Vec<String> {
    let mut keys = Vec::new();
    if let Value::Object(map) = value {
        for key in map.keys() {
            keys.push(key.to_string());
        }
    }
    keys
}

pub fn get_values(value: &Value) -> Vec<Value> {
    let mut values = Vec::new();
    if let Value::Object(map) = value {
        for value in map.values() {
            values.push(value.clone());
        }
    }
    values
}

pub fn contains<T: PartialEq>(val: &T, vec: Vec<T>) -> bool {
    for item in vec {
        if item == *val {
            return true;
        }
    }
    false
}

pub fn zipper<T: IntoIterator, U: IntoIterator>(a: T, b: U) -> Vec<(T::Item, U::Item)> {
    a.into_iter().zip(b).collect()
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn from_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

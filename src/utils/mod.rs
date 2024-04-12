use serde_json::Value;

use crate::constants::config::MAILLIST_PATH;

use self::json::read_json;

pub mod battle_data;
pub mod battle_replay;
pub mod comp;
pub mod crypto;
pub mod game;
pub mod json;
pub mod random;
pub mod rlutils;
pub mod server;

pub fn zipper<T: IntoIterator, U: IntoIterator>(a: T, b: U) -> Vec<(T::Item, U::Item)> {
    a.into_iter().zip(b).collect()
}

pub fn enumerate<T: IntoIterator>(a: T) -> Vec<(usize, T::Item)> {
    a.into_iter().enumerate().collect()
}

pub fn get_item(payload: Value, key: &str) -> (Vec<Value>, i32) {
    let mut items = Vec::new();
    let has_gift = 1;
    let mut mail_data = read_json(MAILLIST_PATH);

    if key != "sysMailIdList" {
        let mail = &payload[&key];
    } else {
    }

    (items, has_gift)
}

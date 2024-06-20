use axum::Json;
use serde_json::Value;

#[allow(clippy::upper_case_acronyms)]
pub(crate) type JSON = Json<Value>;

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

pub fn get_length(value: &Value) -> usize {
    if let Value::Array(array) = value {
        array.len()
    } else {
        0
    }
}

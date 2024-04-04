use core::panic;
use std::fmt::Display;

use base64::{engine::general_purpose::STANDARD, Engine};

pub fn encode<T: Display>(input: T) -> String {
    let mut output_buf = String::new();
    STANDARD.encode_string(input.to_string(), &mut output_buf);
    output_buf
}

pub fn decode<T: Display>(input: T) -> String {
    let mut output_buf = String::new();
    match STANDARD.decode(input.to_string()) {
        Ok(decoded) => match String::from_utf8(decoded) {
            Ok(s) => &output_buf.push_str(&s),
            Err(e) => panic!("Error: {}", e),
        },
        Err(e) => {
            panic!("Error: {}", e)
        }
    };
    output_buf
}

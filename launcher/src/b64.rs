use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};

pub struct B64Decoder {
    data: Vec<u8>,
}

impl B64Decoder {
    pub fn new<T: AsRef<str>>(data: T) -> Self {
        let data = data.as_ref().as_bytes().to_vec();
        Self { data }
    }

    pub fn decode(self) -> Result<String> {
        let engine = STANDARD;
        Ok(String::from_utf8(engine.decode(self.data)?)?)
    }
}

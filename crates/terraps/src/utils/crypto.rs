pub mod base64 {
    use std::fmt::Display;

    use anyhow::Result;
    use base64::{engine::general_purpose::STANDARD, Engine};

    pub fn encode<T: Display>(input: T) -> String {
        let mut output_buf = String::new();
        STANDARD.encode_string(input.to_string(), &mut output_buf);
        output_buf
    }

    pub fn encode_bytes(input: Vec<u8>) -> String {
        STANDARD.encode(input)
    }

    #[allow(dead_code)]
    pub fn decode<T: Display>(input: T) -> Result<String> {
        Ok(String::from_utf8(STANDARD.decode(input.to_string())?)?)
    }

    pub fn decode_bytes<T: Display>(input: T) -> Result<Vec<u8>> {
        Ok(STANDARD.decode(input.to_string())?)
    }

    /// Encrypts the given data using the `base64` algorithm.
    #[allow(dead_code)]
    pub fn encrypt<T: AsRef<[u8]>>(data: T) -> String {
        STANDARD.encode(data)
    }

    /// Decrypts the given data using the `base64` algorithm.
    #[allow(dead_code)]
    pub fn decrypt<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>> {
        Ok(STANDARD.decode(data)?)
    }
}

pub mod md5 {
    use md5::{Digest, Md5};

    pub fn md5_digest(data: &[u8]) -> Vec<u8> {
        let mut hasher = Md5::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    pub fn md5_hexdigest(data: &[u8]) -> String {
        let mut hasher = Md5::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

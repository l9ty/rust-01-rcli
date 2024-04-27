use std::io;

use anyhow::{Ok, Result};

use crate::cli::base64::Base64Method;
use base64::prelude::*;

pub fn process_encode(input: &mut dyn io::Read, method: Base64Method) -> Result<String> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let s = buf.trim();

    let encoded = match method {
        Base64Method::Standard => BASE64_STANDARD.encode(s),
        Base64Method::UrlSafe => BASE64_URL_SAFE.encode(s),
    };

    Ok(encoded)
}

pub fn process_decode(input: &mut dyn io::Read, method: Base64Method) -> Result<String> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let s = buf.trim();

    let decoded = match method {
        Base64Method::Standard => BASE64_STANDARD.decode(s)?,
        Base64Method::UrlSafe => BASE64_URL_SAFE.decode(s)?,
    };

    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

#[cfg(test)]
mod test {

    use crate::utils::open_reader;

    use super::*;

    fn encode_decode(method: Base64Method) {
        let infile = "Cargo.toml";
        let mut reader = open_reader(infile).expect("read content error");
        let encoded = process_encode(reader.as_mut(), method).expect("encode error");
        process_decode(&mut (encoded.as_bytes()), method).expect("process encode error");
    }

    #[test]
    fn t_encode_decode() {
        encode_decode(Base64Method::Standard);
        encode_decode(Base64Method::UrlSafe);
    }
}

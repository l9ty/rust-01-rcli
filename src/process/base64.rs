use std::{fs, io};

use anyhow::{Ok, Result};

use crate::cli::base64::Base64Method;
use base64::prelude::*;

fn read_content(infile: &str) -> Result<Box<dyn io::Read>> {
    if infile == "-" {
        Ok(Box::new(io::stdin()) as Box<dyn io::Read>)
    } else {
        Ok(Box::new(fs::File::open(infile)?) as Box<dyn io::Read>)
    }
}

pub fn process_encode(infile: &str, method: Base64Method) -> Result<()> {
    let mut reader = read_content(infile)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let s = buf.trim();

    let encoded = match method {
        Base64Method::Standard => BASE64_STANDARD.encode(s),
        Base64Method::UrlSafe => BASE64_URL_SAFE.encode(s),
    };

    println!("{}", encoded);

    Ok(())
}

pub fn process_decode(infile: &str, method: Base64Method) -> Result<()> {
    let mut reader = read_content(infile)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let s = buf.trim();

    let decoded = match method {
        Base64Method::Standard => BASE64_STANDARD.decode(s)?,
        Base64Method::UrlSafe => BASE64_URL_SAFE.decode(s)?,
    };

    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_process_encode() {
        let infile = "Cargo.toml";
        let method = Base64Method::Standard;
        assert!(process_encode(infile, method).is_ok());
    }

    #[test]
    fn t_process_decode() {
        let infile = "./features/b64.txt";
        let method = Base64Method::UrlSafe;
        assert!(process_decode(infile, method).is_ok());
    }
}

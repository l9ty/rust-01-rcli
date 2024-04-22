use std::str::FromStr;

use super::verify_path;
use clap::Parser;
use std::fmt::Display;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_path)]
    pub input: String,
    #[arg(short, long, default_value_t = Base64Method::UrlSafe)]
    pub method: Base64Method,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_path)]
    pub input: String,
    #[arg(short, long, default_value_t = Base64Method::UrlSafe)]
    pub method: Base64Method,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Method {
    UrlSafe,
    Standard,
}

impl FromStr for Base64Method {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "urlsafe" => Ok(Base64Method::UrlSafe),
            "standard" => Ok(Base64Method::Standard),
            _ => Err(anyhow::anyhow!("Invalid base64 method: {}", s)),
        }
    }
}

impl From<Base64Method> for &'static str {
    fn from(method: Base64Method) -> Self {
        match method {
            Base64Method::UrlSafe => "urlsafe",
            Base64Method::Standard => "standard",
        }
    }
}

impl Display for Base64Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

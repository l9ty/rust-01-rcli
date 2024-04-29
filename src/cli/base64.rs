use std::str::FromStr;

use crate::{
    process::base64::{process_decode, process_encode},
    utils::open_reader,
    CmdExector,
};

use super::verify_file;
use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::fmt::Display;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, default_value_t = Base64Method::UrlSafe)]
    pub method: Base64Method,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, default_value_t = Base64Method::UrlSafe)]
    pub method: Base64Method,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Method {
    UrlSafe,
    Standard,
}

impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = open_reader(&self.input)?;
        let ret = process_encode(reader.as_mut(), self.method)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = open_reader(&self.input)?;
        let ret = process_decode(reader.as_mut(), self.method)?;
        println!("{}", ret);
        Ok(())
    }
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

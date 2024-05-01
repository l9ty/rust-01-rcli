pub mod base64;
pub mod csv;
pub mod genpass;
pub mod http;
pub mod jwt;
pub mod text;

use std::path::Path;

use crate::{process::csv::process_csv, CmdExector};

pub use self::{
    base64::{Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand},
    csv::CsvOpts,
    genpass::GenPassOpts,
    http::{HttpServeOpts, HttpSubCommand},
    jwt::{JwtSignOpts, JwtSubCommand, JwtVerifyOpts},
    text::{
        TextDecryptOpts, TextEncryptOpts, TextGenerateOpts, TextSignOpts, TextSubCommand,
        TextVerifyOpts,
    },
};

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use regex::Regex;

// rcli csv -i input -o output --header -d ,
#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct CmdOpts {
    #[command(subcommand)]
    pub sub: SubCommand,
}

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
    #[command(subcommand)]
    Jwt(JwtSubCommand),
}

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_csv(&self.input, &self.output, self.format)
    }
}

pub fn verify_file(s: &str) -> Result<String, &'static str> {
    if s == "-" || Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err("File does not exist")
    }
}

pub fn verify_dir(path: &str) -> Result<String, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Directory does not exist")
    }
}

pub struct ReadableDuration(String);

impl ReadableDuration {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl TryInto<i64> for ReadableDuration {
    type Error = ();

    fn try_into(self) -> Result<i64, Self::Error> {
        let re = Regex::new(r"(?:(\d+)([smhd])\s?)+").unwrap();

        let mut delta = 0;

        for (_, [n, unit]) in re.captures_iter(&self.0).map(|c| c.extract()) {
            delta += match unit {
                "s" => n.parse::<i64>().unwrap(),
                "m" => n.parse::<i64>().unwrap() * 60,
                "h" => n.parse::<i64>().unwrap() * 60 * 60,
                "d" => n.parse::<i64>().unwrap() * 60 * 60 * 24,
                _ => unreachable!(),
            }
        }

        if delta == 0 {
            Err(())
        } else {
            Ok(delta)
        }
    }
}

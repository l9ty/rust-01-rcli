pub mod base64;
pub mod csv;
pub mod genpass;
pub mod http_serve;
pub mod text;

use std::path::Path;

use crate::{process::csv::process_csv, CmdExector};

pub use self::{
    base64::{Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand},
    csv::CsvOpts,
    genpass::GenPassOpts,
    http_serve::{HttpServeOpts, HttpSubCommand},
    text::{
        TextDecryptOpts, TextEncryptOpts, TextGenerateOpts, TextSignOpts, TextSubCommand,
        TextVerifyOpts,
    },
};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

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

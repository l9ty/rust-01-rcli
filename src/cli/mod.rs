pub mod base64;
pub mod csv;
pub mod genpass;

use std::path::Path;

pub use self::{base64::Base64SubCommand, csv::CsvOpts, genpass::GenPassOpts};
use clap::{Parser, Subcommand};

// rcli csv -i input -o output --header -d ,
#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct CmdOpts {
    #[command(subcommand)]
    pub sub: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

pub fn verify_path(s: &str) -> Result<String, &'static str> {
    if s == "-" {
        return Ok(s.to_string());
    }

    let p = Path::new(s);
    if p.exists() {
        Ok(s.to_string())
    } else {
        Err("File does not exist")
    }
}

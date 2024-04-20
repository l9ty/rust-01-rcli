use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_path)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_path(s: &str) -> Result<String, &'static str> {
    let p = Path::new(s);
    if p.exists() {
        Ok(s.to_string())
    } else {
        Err("File does not exist")
    }
}

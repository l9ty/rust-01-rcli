use anyhow::Ok;
use clap::Parser;
use zxcvbn::zxcvbn;

use crate::{process::genpass::process_genpass, CmdExector};

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,

    #[arg(short, long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
}

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.length,
            self.number,
            self.symbol,
            self.uppercase,
            self.lowercase,
        )?;
        println!("{}", password);

        let estimate = zxcvbn(&password, &[])?;
        eprintln!("Estimated strength: {}", estimate.score());
        Ok(())
    }
}

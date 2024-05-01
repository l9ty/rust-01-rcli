use anyhow::Ok;

use clap::Parser;
use core::result::Result as CoreResult;
use enum_dispatch::enum_dispatch;

use super::verify_file;
use crate::{
    process::jwt::{process_jwt_sign, process_jwt_verify, Claims},
    utils::read_content,
    CmdExector, ReadableDuration,
};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "Sign a JWT")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a JWT")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long, default_value = "homework")]
    sub: String,
    #[arg(long, default_value = "teacher")]
    aud: String,
    #[arg(short, long, default_value = "rcli")]
    iss: String,
    #[arg(short, long, value_parser = parse_jwt_exp, default_value = "14d")]
    exp: u64,
    #[arg(short, long, default_value = "rcli-default-key")]
    key: String,
}

fn parse_jwt_exp(s: &str) -> Result<u64, &'static str> {
    let readable = ReadableDuration::new(s.to_string());
    let delta: i64 = readable.try_into().map_err(|_| "Invalid time format")?;
    let exp = jsonwebtoken::get_current_timestamp() + delta as u64;
    CoreResult::Ok(exp)
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, default_value = "rcli-default-key")]
    pub key: String,
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let claim = Claims {
            sub: self.sub,
            aud: self.aud,
            exp: self.exp,
        };

        let sig = process_jwt_sign(&self.key, &claim)?;
        print!("{}", sig);
        Ok(())
    }
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = read_content(&self.input)?;
        let sig = String::from_utf8(sig)?;
        let mut sig = sig.trim().as_bytes();
        match process_jwt_verify(&mut sig, &self.key) {
            Err(e) => println!("⚠ JWT verification failed: {}", e),
            _ => println!("✓ JWT verified"),
        };
        Ok(())
    }
}

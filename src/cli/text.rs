use anyhow::{Ok, Result};
use std::{fmt::Display, path::PathBuf, str::FromStr};

use super::verify_file;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum TextSubCommand {
    #[command(
        name = "sign",
        about = "Sign a message with a private/session key and return the signature"
    )]
    Sign(TextSignOpts),
    #[command(
        name = "verify",
        about = "Verify a message with a public/session key and signature"
    )]
    Verify(TextVerifyOpts),
    #[command(
        name = "generate",
        about = "Generate a random blake3 key or ed25519 key pair"
    )]
    Generate(TextGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long, default_value_t = TextSignMethod::Blake3, value_parser = parse_text_sign_method)]
    pub method: TextSignMethod,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(short, long, default_value_t = TextSignMethod::Blake3, value_parser = parse_text_sign_method)]
    pub method: TextSignMethod,
}

#[derive(Debug, Parser)]
pub struct TextGenerateOpts {
    #[arg(short, long, default_value_t = TextSignMethod::Blake3, value_parser = parse_text_sign_method)]
    pub method: TextSignMethod,
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignMethod {
    Blake3,
    Ed25519,
}

fn parse_text_sign_method(s: &str) -> Result<TextSignMethod, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignMethod::Blake3),
            "ed25519" => Ok(TextSignMethod::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid method")),
        }
    }
}

impl From<TextSignMethod> for &str {
    fn from(m: TextSignMethod) -> Self {
        match m {
            TextSignMethod::Blake3 => "blake3",
            TextSignMethod::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

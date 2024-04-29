use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use enum_dispatch::enum_dispatch;
use std::{fmt::Display, fs, path::PathBuf, str::FromStr};

use crate::{
    process::text::{
        process_text_encrypt, process_text_key_generate, process_text_sign, process_text_verify,
    },
    utils::{open_reader, read_content},
    CmdExector,
};

use super::{verify_dir, verify_file};
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/session key and return the signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a message with a public/session key and signature")]
    Verify(TextVerifyOpts),
    #[command(about = "Encrypt a message with chacha20poly1305")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt a message with chacha20poly1305")]
    Decrypt(TextDecryptOpts),
    #[command(
        name = "generate",
        about = "Generate a random blake3 key or ed25519 key pair"
    )]
    Generate(TextGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
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
    #[arg(short, long, value_parser = parse_text_sign_method, default_value_t = TextSignMethod::Blake3)]
    pub method: TextSignMethod,
    #[arg(short, long, value_parser = verify_dir, default_value = ".")]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignMethod {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut msg = open_reader(&self.input)?;
        let key = read_content(&self.key)?;
        let sig = process_text_sign(msg.as_mut(), key.as_slice(), self.method)?;
        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut msg = open_reader(&self.input)?;
        let key = read_content(&self.key)?;
        let sig = URL_SAFE_NO_PAD.decode(&self.sig)?;
        let verified =
            process_text_verify(msg.as_mut(), key.as_slice(), sig.as_slice(), self.method)?;
        if verified {
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }
        Ok(())
    }
}

impl CmdExector for TextGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_key_generate(self.method)?;
        for (filename, contents) in key {
            fs::write(self.output.join(filename), contents)?;
        }
        Ok(())
    }
}

impl CmdExector for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut msg = open_reader(&self.input)?;
        let key = hex::decode(&self.key)?;
        let encrypted = process_text_encrypt(msg.as_mut(), key.as_slice(), true)?;
        let encoded = URL_SAFE_NO_PAD.encode(encrypted);
        print!("{}", encoded);
        Ok(())
    }
}

impl CmdExector for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = open_reader(&self.input)?;
        let mut b64msg = Vec::new();
        reader.read_to_end(&mut b64msg)?;
        let msg = URL_SAFE_NO_PAD.decode(&b64msg)?;
        let key = hex::decode(self.key)?;
        let decoded = process_text_encrypt(&mut msg.as_slice(), key.as_slice(), false)?;
        println!("{}", String::from_utf8(decoded)?);
        Ok(())
    }
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

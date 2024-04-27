use std::fs;

use anyhow::{Ok, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;

use rcli::cli::text::TextSubCommand;
use rcli::cli::{Base64SubCommand, CmdOpts, SubCommand};
use rcli::process::text::{process_text_key_generate, process_text_sign, process_text_verify};
use rcli::process::{
    base64::{process_decode, process_encode},
    csv::process_csv,
    genpass::process_genpass,
};
use rcli::utils::{open_reader, reader_content};
use zxcvbn::zxcvbn;

fn main() -> Result<()> {
    let opts = CmdOpts::parse();
    match opts.sub {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, opts.format)?,
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.number,
                opts.symbol,
                opts.uppercase,
                opts.lowercase,
            )?;
            println!("{}", password);

            let estimate = zxcvbn(&password, &[])?;
            eprintln!("Estimated strength: {}", estimate.score());
        }
        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => {
                let mut reader = open_reader(&opts.input)?;
                let ret = process_encode(reader.as_mut(), opts.method)?;
                println!("{}", ret);
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = open_reader(&opts.input)?;
                let ret = process_decode(reader.as_mut(), opts.method)?;
                println!("{}", ret);
            }
        },
        SubCommand::Text(opts) => match opts {
            TextSubCommand::Sign(opts) => {
                let mut msg = open_reader(&opts.input)?;
                let key = reader_content(&opts.key)?;
                let sig = process_text_sign(msg.as_mut(), key.as_slice(), opts.method)?;
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubCommand::Verify(opts) => {
                let mut msg = open_reader(&opts.input)?;
                let key = reader_content(&opts.key)?;
                let sig = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified =
                    process_text_verify(msg.as_mut(), key.as_slice(), sig.as_slice(), opts.method)?;
                if verified {
                    println!("✓ Signature verified")
                } else {
                    println!("⚠ Signature not verified")
                }
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.method)?;
                for (filename, contents) in key {
                    fs::write(opts.output.join(filename), contents)?;
                }
            }
        },
    };

    Ok(())
}

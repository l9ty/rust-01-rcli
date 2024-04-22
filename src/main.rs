use anyhow::Result;
use clap::Parser;

use rcli::cli::{Base64SubCommand, CmdOpts, SubCommand};
use rcli::process::{
    base64::{process_decode, process_encode},
    csv::process_csv,
    genpass::process_genpass,
};

fn main() -> Result<()> {
    let opts = CmdOpts::parse();
    match opts.sub {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, opts.format),
        SubCommand::GenPass(opts) => process_genpass(
            opts.length,
            opts.number,
            opts.symbol,
            opts.uppercase,
            opts.lowercase,
        ),
        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.method),
            Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.method),
        },
    }
}

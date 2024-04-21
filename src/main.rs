use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, process_genpass, CmdOpts, SubCommand};

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
    }
}

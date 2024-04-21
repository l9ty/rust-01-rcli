use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, CmdOpts, SubCommand};

fn main() -> Result<()> {
    let opts = CmdOpts::parse();
    match opts.sub {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, opts.format),
    }
}

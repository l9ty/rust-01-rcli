use clap::Parser;
use rcli::cli::CmdOpts;
use rcli::CmdExector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = CmdOpts::parse();
    opts.sub.execute().await?;
    Ok(())
}

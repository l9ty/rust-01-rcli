use enum_dispatch::enum_dispatch;

pub mod cli;
pub mod process;
pub mod utils;

use cli::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}

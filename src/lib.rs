mod mycli;
mod process;

pub use mycli::{Base64SubCommand, Opts, SubCommand};

pub use process::*;

#[allow(async_fn_in_trait)]
pub trait CmdExcutor {
    async fn execute(self) -> anyhow::Result<()>;
}

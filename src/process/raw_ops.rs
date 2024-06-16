//use crate::mycli::raw::{RawDecodeOpts, RawEncodeOpts, RawFormat, RawSubCommand};
use crate::mycli::raw::{RawFormat, RawSubCommand};

use anyhow::{Ok, Result};

//use std::{fs::File, io::Read};

use crate::CmdExcutor;

impl CmdExcutor for RawSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        unimplemented!();
        //match self {
        //    RawSubCommand::Encode(opts) => opts.execute().await,
        //    RawSubCommand::Decode(opts) => opts.execute().await,
        //}
    }
}

pub fn process_raw_encode(_input: &str, _format: RawFormat) -> Result<()> {
    Ok(())
}

pub fn process_raw_decode(_input: &str, _format: RawFormat) -> Result<()> {
    Ok(())
}

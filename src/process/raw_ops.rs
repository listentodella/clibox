use crate::mycli::raw::{RawDecodeOpts, RawEncodeOpts, RawFormat, RawSubCommand};
use anyhow::{Ok, Result};
use bincode;
use std::{fs::File, io::Read};

use crate::CmdExcutor;

impl CmdExcutor for RawSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            RawSubCommand::Encode(opts) => opts.execute().await,
            RawSubCommand::Decode(opts) => opts.execute().await,
        }
    }
}

impl CmdExcutor for RawDecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        unimplemented!();
    }
}

impl CmdExcutor for RawEncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_raw_encode(&self.input, self.output, self.format)?;
        Ok(())
    }
}

pub fn process_raw_encode(input: &str, output: Option<String>, format: RawFormat) -> Result<()> {
    let output = if let Some(o) = output {
        o.clone()
    } else {
        format!("output.{}", format)
    };
    println!("output file name = {}", output);

    let mut buffer = String::new();
    let mut rfile = File::open(input)?;
    rfile.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    //let mut ret = Vec::new();
    //let mut reader = bincode::Reader::from_path(input)?;
    //let content = match format {
    //    rawFormat::Json => serde_json::to_string_pretty(&ret)?,
    //    rawFormat::Yaml => serde_yaml::to_string(&ret)?,
    //};
    //fs::write(output, content)?;
    //serde_bincode;

    Ok(())
}

pub fn process_raw_decode(_input: &str, _format: RawFormat) -> Result<()> {
    Ok(())
}

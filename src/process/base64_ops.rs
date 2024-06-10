use crate::mycli::base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand};

use anyhow::{Ok, Result};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs::File, io::Read};

use crate::CmdExcutor;

impl CmdExcutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        //let mut reader = get_reader(&self.input)?;
        //let ret = process_base64_decode(&mut reader, self.format)?;
        //todo!();
        process_base64_decode(&self.input, self.format)?;

        Ok(())
    }
}

impl CmdExcutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_base64_encode(&self.input, self.format)?;
        Ok(())
    }
}

impl CmdExcutor for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn process_base64_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut buf = Vec::new();
    let mut reader = get_reader(input)?;
    reader.read_to_end(&mut buf)?;
    let encode = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    println!("{}", encode);

    Ok(())
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut buf = String::new();
    let mut reader = get_reader(input)?;
    reader.read_to_string(&mut buf)?;
    //防止有意外的新行
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    //TODO:仅对于本例,可以当作是string
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);

    Ok(())
}

use crate::mycli::Base64Format;
use anyhow::{Ok, Result};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs::File, io::Read};

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

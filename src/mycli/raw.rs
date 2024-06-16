use anyhow::Ok;
use clap::Parser;
use std::{fmt, str::FromStr};

#[derive(Debug, Parser)]
pub enum RawSubCommand {
    #[command(name = "encode", about = "将目标文件编码成binary raw")]
    Encode(RawEncodeOpts),
    #[command(name = "decode", about = "将目标binary raw解码成文本文件")]
    Decode(RawDecodeOpts),
}

#[derive(Debug, Parser)]
pub struct RawEncodeOpts {
    #[arg(short, long, value_parser = crate::find_filename, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_raw_format, default_value = "standard")]
    pub format: RawFormat,
}

#[derive(Debug, Parser)]
pub struct RawDecodeOpts {
    #[arg(short, long, value_parser = crate::find_filename, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_raw_format, default_value = "standard")]
    pub format: RawFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum RawFormat {
    Bin,
    Rgb888,
    Bgr888,
    Rgba,
    Yuv422,
    Yuv444,
}

fn parse_raw_format(format: &str) -> Result<RawFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for RawFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bin" => Ok(RawFormat::Bin),
            "rgb24" | "rgb888" | "rgb" => Ok(RawFormat::Rgb888),
            "bgr" | "bgr888" => Ok(RawFormat::Bgr888),
            "rgba" => Ok(RawFormat::Rgba),
            "yuv422" => Ok(RawFormat::Yuv422),
            "yuv444" => Ok(RawFormat::Yuv444),
            _ => Err(anyhow::anyhow!("未知格式")),
        }
    }
}

impl From<RawFormat> for &'static str {
    fn from(format: RawFormat) -> Self {
        match format {
            RawFormat::Yuv422 | RawFormat::Yuv444 => "yuv",
            RawFormat::Bgr888 | RawFormat::Rgb888 => "rgb",
            RawFormat::Rgba => "rgba",
            RawFormat::Bin => "bin",
        }
    }
}

impl fmt::Display for RawFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

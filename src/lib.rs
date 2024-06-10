mod mycli;
mod process;

use std::{fmt, path::Path, str::FromStr};

pub use mycli::Opts;
pub use process::*;

#[allow(async_fn_in_trait)]
pub trait CmdExcutor {
    async fn execute(self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn find_filename(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("未找到目标文件")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    //pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
    //看函数原型,想要使用parse(),就必须为目标类型实现FromStr
    //下面为 OutputFormat 实现了 FromStr
    //因此可以使用parse()去解析str,并将其转换为 OutputFormat了
    format.parse()
}

// 为目标类型 实现 From trait, 该类型就可以转换为 str
// 同时意味着, Into trait被自动实现
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// 为目标类型实现 FromStr trait, 那么 str 就可以转换为该类型
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // 使用anyhow宏来定义错误
            _ => Err(anyhow::anyhow!("invalid format")),
        }
    }
}

// 为目标类型实现 Display trait, 那么该类型就可以打印了
// 这意味着, print, format等系列函数/宏可以直接使用该类型
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

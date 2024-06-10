use crate::CmdExcutor;
use clap::Parser;
use std::{fmt, path::Path, str::FromStr};

impl CmdExcutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::Pwd(opts) => opts.execute().await,
            SubCommand::Base64(subcmd) => subcmd.execute().await,
        }
    }
}

//对该复合类型使用clap::Parser派生宏
#[derive(Debug, Parser)]
//使用command宏,解释该命令,如果不主动赋值,会从项目的Cargo.toml里取值
#[command(name = "mycli", version, author, about, long_about = None)]
pub struct Opts {
    //使用subcommand标记该成员为子命令
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    //对子命令成员解释,如果不显式起名,则默认为该成员名的小写
    #[command(name = "csv", about = "展示csv, 或将csv转换为其他格式")]
    Csv(CsvOpts),
    #[command(name = "pwd", about = "随机密码生成器")]
    Pwd(PwdOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

// 对于结构体,必须将每一个想要对外暴露的成员使用pub修饰
#[derive(Debug, Parser)]
pub struct CsvOpts {
    //使用arg宏修饰该成员,允许short,long解析,并提供自己的解析函数
    //如果在上面以文档风格注释,则自动将其转换为help显示的信息
    /// 输入文件名
    #[arg(short, long, value_parser = find_filename )]
    pub input: String,
    //不再通过clap的宏提供默认值,则对于该选项,可以考虑用Option,然后在代码里处理
    /// 输出文件名
    #[arg(short, long)] //"output.json".into()
    pub output: Option<String>,
    // 对于自定义类型,必须提供解析函数
    /// 输出格式
    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    //提供了默认值, 以字符的方式接收
    /// 分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    //提供了默认值, 以bool值的方式接收
    /// 是否有header
    #[arg(long, default_value_t = true)]
    pub header: bool,
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

#[derive(Debug, Parser)]
pub struct PwdOpts {
    // 密码强度检查
    #[arg(short, long, default_value_t = false)]
    pub check: bool,
    /// 随机密码的长度,默认16
    #[arg(short = 'L', long, default_value_t = 16)]
    pub len: u8,
    /// 随机密码是否包含大写,默认无
    #[arg(short, long, default_value_t = false)]
    pub uppercase: bool,
    /// 随机密码是否包含小写,默认无
    #[arg(short, long, default_value_t = false)]
    pub lowercase: bool,
    /// 随机密码是否包含数字,默认无
    #[arg(short, long, default_value_t = false)]
    pub number: bool,
    /// 随机密码是否包含标点符号,默认无
    #[arg(short, long, default_value_t = false)]
    pub symbol: bool,
}

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "将一个字符串编码成base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "将一个base64进行解码")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = find_filename, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = find_filename, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("未知格式")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

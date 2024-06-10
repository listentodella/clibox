pub mod base64;
pub mod csv;
pub mod pwd;

use crate::CmdExcutor;
use base64::Base64SubCommand;
use clap::Parser;
use csv::CsvOpts;
use pwd::PwdOpts;

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

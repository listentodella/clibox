use clap::Parser;
use std::path::Path;

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
}

// 对于结构体,必须将每一个想要对外暴露的成员使用pub修饰
#[derive(Debug, Parser)]
pub struct CsvOpts {
    //使用arg宏修饰该成员,允许short,long解析,并提供自己的解析函数
    //如果在上面以文档风格注释,则自动将其转换为help显示的信息
    /// 输入文件名
    #[arg(short, long, value_parser = find_filename )]
    pub input: String,
    //提供了默认值, 以字符串的方式接收
    /// 输出文件名
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
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

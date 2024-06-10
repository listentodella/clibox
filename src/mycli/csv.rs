use crate::OutputFormat;
use clap::Parser;

// 对于结构体,必须将每一个想要对外暴露的成员使用pub修饰
#[derive(Debug, Parser)]
pub struct CsvOpts {
    //使用arg宏修饰该成员,允许short,long解析,并提供自己的解析函数
    //如果在上面以文档风格注释,则自动将其转换为help显示的信息
    /// 输入文件名
    #[arg(short, long, value_parser = crate::find_filename )]
    pub input: String,
    //不再通过clap的宏提供默认值,则对于该选项,可以考虑用Option,然后在代码里处理
    /// 输出文件名
    #[arg(short, long)] //"output.json".into()
    pub output: Option<String>,
    // 对于自定义类型,必须提供解析函数
    /// 输出格式
    #[arg(short, long, value_parser = crate::parse_format, default_value = "json")]
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

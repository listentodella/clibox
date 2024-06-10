//mycli csv -i -o output.json --header -d ','
use clap::Parser;
use mycli::{CmdExcutor, Opts};

//告诉编译器使用tokio作为异步运行时
#[tokio::main]
//函数前面加上 async,它就会变成由异步运行时安排的异步任务
async fn main() -> anyhow::Result<()> {
    //tracing库在Rust中用于生成日志和追踪信息
    //然而它本身并不负责将这些信息打印或记录到某个地方
    //这个任务是由所谓的“订阅者”（Subscriber）或“收集器”（Collector）来完成的
    //tracing-subscriber正是这样一个库，它提供了多种“订阅者”的实现
    //用于收集,处理和输出tracing生成的日志和追踪信息
    tracing_subscriber::fmt::init();
    //let opts = Opts::parse();
    //opts.cmd.execute().await?;
    //只有遇到 await 关键字,它对应的async函数才会被执行
    Opts::parse().cmd.execute().await?;

    Ok(())
}

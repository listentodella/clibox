use clap::Parser;

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

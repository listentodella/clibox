//mycli csv -i -o output.json --header -d ','
use clap::Parser;
use mycli::{
    process_base64_decode, process_base64_encode, process_csv, process_pwd, Base64SubCommand, Opts,
    SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            println!("get opts {:?}", csv_opts);
            process_csv(&csv_opts.input, csv_opts.output, csv_opts.format)?;
        }
        SubCommand::Pwd(pwd_opts) => process_pwd(
            pwd_opts.check,
            pwd_opts.len,
            pwd_opts.uppercase,
            pwd_opts.lowercase,
            pwd_opts.number,
            pwd_opts.symbol,
        )?,
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => process_base64_encode(&opts.input, opts.format)?,
            Base64SubCommand::Decode(opts) => process_base64_decode(&opts.input, opts.format)?,
        },
    }

    Ok(())
}

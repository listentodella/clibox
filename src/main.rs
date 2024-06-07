//mycli csv -i -o output.json --header -d ','
use clap::Parser;
use mycli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            println!("get opts {:?}", csv_opts);
            process_csv(&csv_opts.input, &csv_opts.output)?;
        }
    }

    Ok(())
}

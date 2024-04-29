// use std::process::Output;

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    Subcommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // print!("{:?}", opts);

    match opts.cmd {
        Subcommand::Csv(opts) => {
            // let mut reader = Reader::from_path(opts.input)?;
            // let records = reader
            //     .deserialize()
            //     .map(|record| record.unwrap())
            //     .collect::<Vec<Player>>();
            // print!("{:?} ", records)
            // let mut ret: Vec<Player> = Vec::with_capacity(128);
            // for result in reader.deserialize() {
            //     let record: Player = result?;
            //     // print!("{:?} ", record);
            //     ret.push(record);
            // }
            // let json = serde_json::to_string_pretty(&ret)?;
            // fs::write(opts.output, json)?;
            let output: String = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        Subcommand::GenPass(opts) => {
            process_genpass(&opts)?;
        }
        Subcommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}

// use std::process::Output;

use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

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
    }
    Ok(())
}

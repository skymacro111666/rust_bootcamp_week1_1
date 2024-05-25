// use std::process::Output;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_http_serve, process_text_key_generate, process_text_sign, process_text_verify,
    Base64SubCommand, HttpSubCommand, Opts, Subcommand, TextSubCommand,
};
use std::fs;
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
            let ret = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", ret);

            // output password strength in stderr
            let estimate = zxcvbn(&ret, &[])?;
            eprintln!("Password strength: {}", estimate.score());
        }
        Subcommand::Base64(cmd) => match cmd {
            Base64SubCommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let ret = process_encode(&mut reader, opts.format)?;
                println!("{}", ret);
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let ret = process_decode(&mut reader, opts.format)?;
                println!("{}", ret);
            }
        },
        Subcommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                // base64 output
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
        Subcommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opts) => {
                // println!("Serving at http://0.0.0.0:{}", opts.port);
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}

// RUST_LOG=info cargo run --http serve

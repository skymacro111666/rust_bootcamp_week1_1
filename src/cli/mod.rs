mod base64;
mod csv;
mod genpass;
mod http;
mod text;
use crate::CmdExector;
use clap::Parser;
use std::path::{Path, PathBuf};

use self::csv::CsvOpts;
pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    genpass::GenPassOpts,
    http::HttpSubCommand,
    text::{TextSignFormat, TextSubCommand},
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

impl CmdExector for Subcommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Subcommand::Csv(opts) => opts.execute().await,
            Subcommand::GenPass(opts) => opts.execute().await,
            Subcommand::Base64(cmd) => cmd.execute().await,
            Subcommand::Text(cmd) => cmd.execute().await,
            Subcommand::Http(cmd) => cmd.execute().await,
        }
    }
}

fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist".into()));
    }
}

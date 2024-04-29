use clap::Parser;
use std::{fmt, str::FromStr};

use super::verify_input_file;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_format, default_value = "output.json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // match format.to_lowercase().as_str() {
    //     "json" => Ok(OutputFormat::Json),
    //     "yaml" => Ok(OutputFormat::Yaml),
    //     "toml" => Ok(OutputFormat::Toml),
    //     _ => Err("Invalid format"),
    // }
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Unsupported format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

// impl TryFrom<&str> for OutputFormat {
//     type Error = anyhow::Error;

//     fn try_from(format: &str) -> Result<Self, Self::Error> {
//         match format.to_lowercase().as_str() {
//             "json" => Ok(OutputFormat::Json),
//             "yaml" => Ok(OutputFormat::Yaml),
//             "toml" => Ok(OutputFormat::Toml),
//             v => anyhow::bail!("Unsupported format {}", v),
//         }
//     }
// }

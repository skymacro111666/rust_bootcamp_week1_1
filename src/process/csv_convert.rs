use crate::cli::OutputFormat;
use anyhow::{Ok, Result};
use csv::{Reader, StringRecord};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
// use std::fs::{self, File};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader: Reader<fs::File> = Reader::from_path(input)?;
    let mut ret: Vec<Value> = Vec::with_capacity(128);
    let headers: StringRecord = reader.headers()?.clone();
    for result in reader.records() {
        let record: StringRecord = result?;
        let json_value: Value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        // print!("{:?} ", json_value);

        ret.push(json_value);
    }

    let content: String = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}

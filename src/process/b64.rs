use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::Base64Format;
// use serde_json::de::Read;
use std::{fs::File, io::Read};

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("encode: {}", encode);
    Ok(encode)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    println!("decode: {}", String::from_utf8(decoded.clone())?);
    Ok(String::from_utf8(decoded)?)
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_encode() -> Result<()> {
        let format = Base64Format::Standard;
        let input = "Cargo.toml";
        assert!(process_encode(input, format).is_ok());
        Ok(())
    }
    #[test]
    fn test_process_decode() -> Result<()> {
        let format = Base64Format::UrlSafe;
        let input = "fixtures/b64.txt";
        assert!(process_decode(input, format).is_ok());
        Ok(())
    }
}

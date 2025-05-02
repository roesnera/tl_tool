use std::io::{Read, Write};
use anyhow::Result;
use base64::{prelude::BASE64_STANDARD, Engine};
use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value=None)]
    input: Option<String>,
    #[arg(short, long, action=ArgAction::SetTrue)]
    decode: bool,
    #[arg(long, action=ArgAction::SetTrue)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut input = String::new();

    if let Some(content) = args.input {
        input = content;
    } else {
        let _ = std::io::stdin().read_to_string(&mut input);
    }

    let decoded = match args.decode {
        true => decode_in(input)?,
        false => encode_in(input)?
    };

    let _ = std::io::stdout().write(decoded.as_bytes());

    Ok(())
}

fn decode_in(inp: String) -> Result<String> {
    let decoded = BASE64_STANDARD.decode(inp)?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}
fn encode_in(inp: String) -> Result<String> {
    let decoded = BASE64_STANDARD.encode(inp.as_bytes());
    Ok(decoded)
}

#[test]
fn decode_str_test() {
    let input = String::from("aGVxbG8=");
    let decoded = decode_in(input);

    assert!(decoded.is_ok());
}

#[test]
fn encode_str_test() {
    let input = String::from("hello");

    assert!(encode_in(input).is_ok());
}

#[test]
fn bidirectional() {
    let original = String::from("hello");
    let encoded = encode_in(original);
    assert!(encoded.is_ok());

    let encoded = encoded.unwrap();
    let decoded = decode_in(encoded);
    assert!(decoded.is_ok());

    let decoded = decoded.unwrap();
    assert_eq!(decoded, "hello");
}

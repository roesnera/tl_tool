use std::io::Read;
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

    if args.debug { println!("{:?}", args); }

    let mut input = String::new();

    if let Some(content) = args.input {
        input = content;
    } else {
        println!("else condition reached");
        let _ = std::io::stdin().read_to_string(&mut input);
    }

    println!("{}", input);

    let decoded = match args.decode {
        true => decode_in(input)?,
        false => encode_in(input)?
    };

    print!("{}", decoded);

    Ok(())
}

fn decode_in(inp: String) -> Result<String> {
    let decoded = BASE64_STANDARD.decode(inp)?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}
fn encode_in(inp: String) -> Result<String> {
    let decoded = BASE64_STANDARD.encode(inp);
    Ok(decoded)
}

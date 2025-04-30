use anyhow::Result;
use base64::{prelude::BASE64_STANDARD, Engine};
use clap::{ArgAction, Parser};
use clap_stdin::FileOrStdin;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: FileOrStdin,
    #[arg(short, long, default_value = "std_out")]
    out: String,
    #[arg(short, long, action=ArgAction::SetTrue)]
    decode: bool,
    #[arg(short, long, action=ArgAction::SetTrue)]
    json: bool,
    #[arg(long, action=ArgAction::SetTrue)]
    debug: bool,
}

enum CustomError {
    ArgumentConflct,
}


fn main() -> Result<()> {
    let args = Args::parse();

    if args.debug { println!("{:?}", args); }

    if args.decode && args.json {
        panic!("Invalid arguments")
    }

    let input = args.input.contents()?;

    let input = match args.json {
        true => parse_json(input)?,
        false => input
    };

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
fn parse_json(inp: String) -> Result<String> {
    println!("{}", inp);

    Ok(inp)
}

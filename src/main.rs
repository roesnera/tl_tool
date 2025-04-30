use anyhow::Result;
use base64::{prelude::BASE64_STANDARD, Engine};
use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,
    #[arg(short, long, default_value = "std_out")]
    out: String,
    #[arg(short, long, action=ArgAction::SetTrue)]
    decode: bool,
    #[arg(short, long, action=ArgAction::SetTrue)]
    json: bool
}

impl Args {
    fn format_string(&self) -> String {
        format!("input: {}\nout: {}\ndecode: {}\njson: {}\n", self.input, self.out, self.decode, self.json)
    }
}

enum CustomError {
    ArgumentConflct,
}


fn main() -> Result<()> {
    let args = Args::parse();

    print!("{}", args.format_string());

    if args.decode && args.json {
        panic!("Invalid arguments")
    }

    let decoded = match args.decode {
        true => decode_in(args.input)?,
        false => encode_in(args.input)?
    };

    println!("{}", decoded);
    println!("{:?}", String::from_utf8(BASE64_STANDARD.decode("dGV4dA==")?).unwrap());

    Ok(())
}

fn decode_in(inp: String) -> Result<String> {
    println!("decoding {}", inp);
    let decoded = BASE64_STANDARD.decode(inp.as_bytes())?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}
fn encode_in(inp: String) -> Result<String> {
    let decoded = BASE64_STANDARD.encode(inp);
    Ok(decoded)
}

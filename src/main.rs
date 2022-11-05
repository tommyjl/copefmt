use clap::{arg, Parser};
use copefmt::Formatter;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Parser)]
pub struct CopeFmtArgs {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = CopeFmtArgs::parse();

    let input: Box<dyn Read> = match args.input {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(std::io::stdin()),
    };

    let output: Box<dyn Write> = match args.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(std::io::stdout()),
    };

    Formatter::new(input, output).format()
}

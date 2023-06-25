mod hay;

use std::{fs::read_to_string, path::PathBuf};
use crate::hay::Hay;
use clap::Parser;

#[derive(Parser)]
struct Args {

    /// Length of sequence-matching string
    #[arg(short, long, default_value_t = 5)]
    sequence: usize,

    /// Length of output
    #[arg(short, long, default_value_t = 1000)]
    length: usize,

    /// Display output character-by-character as it's generated
    #[arg(short, long)]
    characters: bool,

    /// Path to write output to; if omitted, will write to STDOUT
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Path to read input text from
    input: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let input = read_to_string(args.input)?;
    let hay = Hay::new(&input);

    let output = hay.generate_output(args.length, args.sequence, args.characters);

    if args.characters {
        println!();
    } else {
        println!("{output}");
    }

    Ok(())
}

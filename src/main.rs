mod hay;

use std::{
    fs::{read_to_string, write},
    ops::RangeInclusive,
    path::PathBuf,
};
use crate::hay::Hay;
use clap::Parser;

const SEQ_RANGE: RangeInclusive<u8> = 1..=10;

#[derive(Parser)]
struct Args {

    /// Length of sequence-matching string.
    #[arg(short, long = "sequence", value_parser = seq_in_range)]
    sequence_length: Option<u8>,

    /// Use words instead of characters as tokens.
    #[arg(short, long = "words", default_value_t = false)]
    word_tokens: bool,

    /// Length of output.
    #[arg(short, long, default_value_t = 1000)]
    length: usize,

    /// Path to write output to. If omitted, will write to STDOUT.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Path to read input text from.
    input: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let input = read_to_string(args.input)?;
    let sequence_length = if let Some(len) = args.sequence_length {
        len
    } else if args.word_tokens { 2 } else { 5 };
    let hay = Hay::new(&input, sequence_length, args.word_tokens);

    let output = hay.generate_output(args.length);

    if let Some(path) = args.output {
        write(path, output)?;
    } else {
        println!("{output}");
    }

    Ok(())
}

fn seq_in_range(s: &str) -> Result<u8, String> {
    let seq: u8 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a number"))?;
    if seq == 0 {
        return Err("sequence length cannot be 0".to_string())
    }
    if SEQ_RANGE.contains(&seq) {
        Ok(seq)
    } else {
        Err(format!("sequence length not in range {}-{}", SEQ_RANGE.start(), SEQ_RANGE.end()))
    }
}

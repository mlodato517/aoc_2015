//! Takes an input file path and writes the compressed contents
//! to `{file_name}.compressed`.
//!
//! Used for compressing Advent of Code inputs because why not.

use clap::Parser;
use std::fs::File;

#[derive(Parser)]
#[clap(about = "Compresses input file to .compressed file")]
struct Args {
    /// Path of file to compress.
    #[clap(long, short)]
    input: std::path::PathBuf,
}
fn main() {
    let args = Args::parse();

    let input_file = File::open(&args.input).unwrap();

    let mut output_path = args.input;
    output_path.set_extension("compressed");
    let output_file = File::create(&output_path).unwrap();

    compress::compress_to(input_file, output_file);

    println!("Wrote to {output_path:?}");
}

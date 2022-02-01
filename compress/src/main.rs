//! Takes an input file path and writes the compressed contents
//! to `{file_name}.compressed`.
//!
//! Used for compressing Advent of Code inputs because why not.

use clap::Parser;

#[derive(Parser)]
#[clap(about = "Compresses input file to .compressed file")]
struct Args {
    /// Path of file to compress.
    #[clap(long, short)]
    input: std::path::PathBuf,
}
fn main() {
    let args = Args::parse();

    let input_file = std::fs::read_to_string(&args.input).unwrap();
    let compressed = compress::compress(input_file);

    let mut output_path = args.input;
    output_path.set_extension("compressed");
    std::fs::write(&output_path, compressed).unwrap();

    println!("Wrote to {:?}", output_path);
}

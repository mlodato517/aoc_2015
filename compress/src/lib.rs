use std::io::{Read, Write};

/// Compresses the passed string into the output byte vector.
pub fn compress_to<R, W>(mut input: R, mut output: W)
where
    R: Read,
    W: Write,
{
    brotli::BrotliCompress(&mut input, &mut output, &Default::default()).unwrap();
}

#[macro_export]
macro_rules! aoc_input {
    ($path:literal) => {{
        let file = include_bytes!($path).to_vec();
        compress::decompress(file.as_slice())
    }};
}

/// Decompresses the passed reader into a string.
///
/// Panics if the result of decompression is invalid UTF-8.
pub fn decompress<R>(mut input: R) -> String
where
    R: Read,
{
    let mut output = Vec::new();
    brotli::BrotliDecompress(&mut input, &mut output).unwrap();
    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_inverts_compress() {
        let s = "some string";
        let mut output = Vec::new();
        compress_to(s.as_bytes(), &mut output);

        assert_eq!(decompress(output.as_slice()), s);
    }
}

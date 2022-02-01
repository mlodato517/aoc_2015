/// Compresses the passed string into the output byte vector.
pub fn compress(input: String) -> Vec<u8> {
    let input = input.into_bytes();
    let mut output = Vec::with_capacity(input.len());
    brotli::BrotliCompress(&mut input.as_slice(), &mut output, &Default::default()).unwrap();
    output
}

#[macro_export]
macro_rules! aoc_input {
    ($path:literal) => {{
        let file = include_bytes!("../input.compressed").to_vec();
        compress::decompress(file)
    }};
}

/// Decompresses the passed byte vector into a string.
pub fn decompress(input: Vec<u8>) -> String {
    let mut output = Vec::with_capacity(input.len());
    brotli::BrotliDecompress(&mut input.as_slice(), &mut output).unwrap();
    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_inverts_compress() {
        let s = "some string";

        assert_eq!(decompress(compress(s.to_string())), s);
    }
}

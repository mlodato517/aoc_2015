/// Count the number of nice strings.
pub fn part1(input: &str) -> usize {
    input.lines().filter(|&line| is_part1_nice(line)).count()
}

/// A string is nice if:
///
/// - it has three vowels (of 'aeiou') (repeated vowels count)
/// - it has at least one letter that appears twice in a row
/// - does not contain 'ab', 'cd', 'pq', or 'xy'
fn is_part1_nice(string: &str) -> bool {
    let mut bytes = string.bytes();
    let mut previous_byte = bytes.next().unwrap();

    let mut has_one_letter_duplicated = false;

    // TODO: Does branchless matter here? Only happens once...
    let mut num_vowels = if is_vowel(previous_byte) { 1 } else { 0 };

    for next_byte in bytes {
        match (previous_byte, next_byte) {
            // If we contain any danger string, return false immediately.
            (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y') => return false,

            // TODO: Does `has_one_letter_duplicated ||= a == b` do anything? It'd remove the
            // match arm below which would be nice.
            // TODO: Does not checking this after we have a duplicated letter help?
            (a, b) if a == b => has_one_letter_duplicated = true,

            _ => { /* nothing to do */ }
        }
        // TODO: Does branchless help here?
        // TODO: Does stopping checking after we have 3 of these help here?
        if is_vowel(next_byte) {
            num_vowels += 1;
        }
        previous_byte = next_byte;
    }
    has_one_letter_duplicated && num_vowels >= 3
}

/// Checks if a byte is the ASCII byte for 'a', 'e', 'i', 'o', or 'u'.
fn is_vowel(byte: u8) -> bool {
    // This is the same machine code as a bunch of `== ||` checks or a `matches!`.
    // See https://godbolt.org/z/nhGq18j6j. Gotta love compilers.
    b"aeiou".contains(&byte)
}

/// Count the number of nice strings using a different "nice" metric.
pub fn part2(input: &str) -> usize {
    input.lines().filter(|&line| is_part2_nice(line)).count()
}

/// A string is nice if:
///
/// - it has a pair of two letters that appear twice in the string without overlapping
/// - it contains at least one letter that repeats with one letter in the middle
fn is_part2_nice(string: &str) -> bool {
    let has_second_property = string
        .as_bytes()
        .windows(3)
        .any(|window| window[0] == window[2]);

    if !has_second_property {
        return false;
    }

    for start in 0..string.len() - 2 {
        let needle = &string.as_bytes()[start..start + 2];
        let has_first_property = string.as_bytes()[start + 2..]
            .windows(2)
            .any(|window| window == needle);

        if has_first_property {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert!(is_part1_nice("ugknbfddgicrmopn"));
        assert!(is_part1_nice("aaa"));

        assert!(!is_part1_nice("jchzalrnumimnmhp"));
        assert!(!is_part1_nice("haegwjzuvuyypxyu"));
        assert!(!is_part1_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part1_mark_input() {
        assert_eq!(part1(include_str!("../input.txt")), 236);
    }

    #[test]
    fn part2_examples() {
        assert!(is_part2_nice("qjhvhtzxzqqjkmpb"));
        assert!(is_part2_nice("xxyxx"));
        assert!(!is_part2_nice("uurcxstgmygtbstg"));
        assert!(!is_part2_nice("ieodomkazucvgmuy"));
    }

    #[test]
    fn part2_mark_input() {
        assert_eq!(part2(include_str!("../input.txt")), 51);
    }
}

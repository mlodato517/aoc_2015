/// Count the number of nice strings.
pub fn part1(input: &str) -> usize {
    input.lines().filter(|&line| is_part1_nice(line)).count()
}

#[derive(Debug)]
struct ParserState {
    num_vowels: u8,
    has_double_char: bool,
    previous_character: char,
}

/// A string is nice if:
///
/// - it has three vowels (of 'aeiou') (repeated vowels count)
/// - it has at least one letter that appears twice in a row
/// - does not contain 'ab', 'cd', 'pq', or 'xy'
fn is_part1_nice(string: &str) -> bool {
    let mut chars = string.chars();
    let mut state = match chars.next() {
        Some(first_char) => {
            let num_vowels = if ['a', 'e', 'i', 'o', 'u'].contains(&first_char) {
                1
            } else {
                0
            };
            ParserState {
                num_vowels,
                has_double_char: false,
                previous_character: first_char,
            }
        }
        None => return false,
    };

    for next_char in chars {
        match (state.previous_character, next_char) {
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
            (_, 'a') | (_, 'e') | (_, 'i') | (_, 'o') | (_, 'u') => state.num_vowels += 1,
            _ => {}
        }
        if state.previous_character == next_char {
            state.has_double_char = true;
        }
        state.previous_character = next_char;
    }

    state.num_vowels >= 3 && state.has_double_char
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
}

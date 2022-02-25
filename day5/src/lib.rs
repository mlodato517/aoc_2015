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
    let has_danger_strings = string
        .as_bytes()
        .windows(2)
        .any(|window| window == b"ab" || window == b"cd" || window == b"pq" || window == b"xy");
    if has_danger_strings {
        return false;
    }

    let has_one_letter_duplicated = string
        .as_bytes()
        .windows(2)
        .any(|window| window[0] == window[1]);
    if !has_one_letter_duplicated {
        return false;
    }

    let mut vowel_count = 0;
    for c in string.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
            vowel_count += 1;
        }
        if vowel_count >= 3 {
            return true;
        }
    }
    false
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

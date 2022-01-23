/// Santa starts on floor 0.
const INITIAL_FLOOR: i32 = 0;

/// Updates the floor Santa would reach after following the passed `instruction`.
/// Returns `Some(new_floor)` to be compatible with [`std::iter::Iterator::scan`].
fn process_floor(current_floor: &mut i32, instruction: char) -> Option<i32> {
    match instruction {
        '(' => *current_floor += 1,
        ')' => *current_floor -= 1,
        _ => panic!("invalid input"),
    }
    Some(*current_floor)
}

/// Returns the last floor Santa would be on after following all the instructions in `input`.
pub fn part1(input: &str) -> i32 {
    input
        .chars()
        .scan(INITIAL_FLOOR, process_floor)
        .last()
        .unwrap()
}

/// Returns the first day Santa be enter the basement on while following all the instructions in
/// `input`.
pub fn part2(input: &str) -> usize {
    input
        .chars()
        .scan(INITIAL_FLOOR, process_floor)
        .zip(1..)
        .find(|&(floor, _)| floor < 0)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_examples() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn test_part_2_examples() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }

    #[test]
    fn test_part_1_chris_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 280);
    }

    #[test]
    fn test_part_2_chris_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1797);
    }
}

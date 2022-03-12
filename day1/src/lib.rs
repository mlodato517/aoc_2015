/// Santa starts on floor 0.
const INITIAL_FLOOR: i32 = 0;

/// Updates the floor Santa would reach after following the passed `instruction`.
/// Returns `Some(new_floor)` to be compatible with [`std::iter::Iterator::scan`].
fn process_floor(current_floor: &mut i32, instruction: u8) -> Option<i32> {
    // We want to add 1 if the floor is ( and subtract 1 for ).
    // ( == 40 and ) == 41 so we can take the instruction, double it
    // to get 80 or 82, and then subtract 81 (which is 40 + 41) to get -1 or 1.
    // We then want to subtract this from the current floor since we want to add 1
    // for ( and subtract 1 for ).
    *current_floor -= instruction as i32 * 2 - (b'(' as i32 + b')' as i32);
    Some(*current_floor)
}

/// Returns the last floor Santa would be on after following all the instructions in `input`.
///
/// NOTE - I tried this with more-or-less `bytes().sum() * 2 - (81 * input.len())` and there was no
/// appreciable performance change.
pub fn part1(input: &str) -> i32 {
    input
        .bytes()
        .scan(INITIAL_FLOOR, process_floor)
        .last()
        .unwrap()
}

/// Returns the first day Santa would enter the basement while following all the instructions in
/// `input`.
pub fn part2(input: &str) -> usize {
    input
        .bytes()
        .scan(INITIAL_FLOOR, process_floor)
        .zip(1..)
        .find_map(|(floor, day)| (floor < 0).then(|| day))
        .unwrap()
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

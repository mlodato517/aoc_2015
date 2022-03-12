use std::collections::HashSet;

/// Find out all the houses visited by Santa by following the directions in `input`.
pub fn part1(input: &str) -> u32 {
    let mut location = (0, 0);
    let mut visited_locations = [location].into_iter().collect::<HashSet<_>>();

    for c in input.chars() {
        location = process_direction(location, c);
        visited_locations.insert(location);
    }
    visited_locations.len() as u32
}

/// Find out all the houses visited by Santa and Robo Santa by following the directions in `input`.
/// One Santa follows every odd direction and the other Santa follows every even direction.
pub fn part2(input: &str) -> u32 {
    let mut location = (0, 0);
    let mut visited_locations = [location].into_iter().collect::<HashSet<_>>();

    // Robo santa loop
    for c in input.chars().step_by(2) {
        location = process_direction(location, c);
        visited_locations.insert(location);
    }

    // Santa loop
    let mut location = (0, 0);
    for c in input.chars().skip(1).step_by(2) {
        location = process_direction(location, c);
        visited_locations.insert(location);
    }
    visited_locations.len() as u32
}

/// Follow a direction to a new location. The directions are given as ASCII characters
/// and direct the Santa north, south, east, or west.
fn process_direction(current_location: (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        '>' => (current_location.0 + 1, current_location.1),
        'v' => (current_location.0, current_location.1 - 1),
        '<' => (current_location.0 - 1, current_location.1),
        '^' => (current_location.0, current_location.1 + 1),
        _ => unreachable!("Invalid char"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_examples() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn part_1_mark_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 2081);
    }

    #[test]
    fn test_part_2_examples() {
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }

    #[test]
    fn part_2_mark_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2341);
    }
}

use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 119] = initialize_directions();

const fn initialize_directions() -> [(i32, i32); 119] {
    let mut directions = [(0, 0); 119];

    directions[b'>' as usize] = (1, 0);
    directions[b'<' as usize] = (-1, 0);
    directions[b'v' as usize] = (0, -1);
    directions[b'^' as usize] = (0, 1);
    directions
}

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
    let mut location1 = (0, 0);
    let mut location2 = (0, 0);
    let mut visited_locations = [location1].into_iter().collect::<HashSet<_>>();

    for (i, c) in input.chars().enumerate() {
        if i & 1 == 0 {
            location1 = process_direction(location1, c);
            visited_locations.insert(location1);
        } else {
            location2 = process_direction(location2, c);
            visited_locations.insert(location2);
        }
    }
    visited_locations.len() as u32
}

/// Follow a direction to a new location. The directions are given as ASCII characters
/// and direct the Santa north, south, east, or west.
fn process_direction(mut location: (i32, i32), direction: char) -> (i32, i32) {
    location.0 += DIRECTIONS[direction as u32 as usize].0;
    location.1 += DIRECTIONS[direction as u32 as usize].1;
    location
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

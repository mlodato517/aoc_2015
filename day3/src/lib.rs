use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let mut location = (0, 0);
    let mut visited_locations = [location].into_iter().collect::<HashSet<_>>();

    for c in input.chars() {
        match c {
            '>' => {
                location.0 += 1;
                visited_locations.insert(location);
            }
            'v' => {
                location.1 -= 1;
                visited_locations.insert(location);
            }
            '<' => {
                location.0 -= 1;
                visited_locations.insert(location);
            }
            '^' => {
                location.1 += 1;
                visited_locations.insert(location);
            }
            _ => unreachable!("Invalid char"),
        }
    }
    visited_locations.len() as u32
}

pub fn part2(input: &str) -> u32 {
    let mut location = (0, 0);
    let mut visited_locations = [location].into_iter().collect::<HashSet<_>>();

    // Robo santa loop
    for c in input.chars().step_by(2) {
        match c {
            '>' => {
                location.0 += 1;
                visited_locations.insert(location);
            }
            'v' => {
                location.1 -= 1;
                visited_locations.insert(location);
            }
            '<' => {
                location.0 -= 1;
                visited_locations.insert(location);
            }
            '^' => {
                location.1 += 1;
                visited_locations.insert(location);
            }
            _ => unreachable!("Invalid char"),
        }
    }

    // Santa loop
    let mut location = (0, 0);
    for c in input.chars().skip(1).step_by(2) {
        match c {
            '>' => {
                location.0 += 1;
                visited_locations.insert(location);
            }
            'v' => {
                location.1 -= 1;
                visited_locations.insert(location);
            }
            '<' => {
                location.0 -= 1;
                visited_locations.insert(location);
            }
            '^' => {
                location.1 += 1;
                visited_locations.insert(location);
            }
            _ => unreachable!("Invalid char"),
        }
    }
    visited_locations.len() as u32
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

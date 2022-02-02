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

pub fn part2(_input: &str) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_examples() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v^v"), 2);
    }

    #[test]
    fn part_1_mark_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 2081);
    }
}

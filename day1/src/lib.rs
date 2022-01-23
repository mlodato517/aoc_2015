use std::str::Chars;

struct FloorIter<'a> {
    floor: i32,
    instructions: Chars<'a>,
}

impl<'a> FloorIter<'a> {
    fn new(s: &'a str) -> Self {
        FloorIter {
            floor: 0,
            instructions: s.chars(),
        }
    }
}

impl Iterator for FloorIter<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.instructions.next().map(|c| {
            match c {
                '(' => self.floor += 1,
                ')' => self.floor -= 1,
                _ => panic!("invalid input"),
            }
            self.floor
        })
    }
}

pub fn part1(input: &str) -> i32 {
    FloorIter::new(input).last().unwrap()
}

pub fn part2(input: &str) -> usize {
    FloorIter::new(input)
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

use std::fs::read_to_string;
use std::str::Chars;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

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

fn part1(input: &str) -> i32 {
    FloorIter::new(input).last().unwrap()
}

fn part2(input: &str) -> usize {
    FloorIter::new(input)
        .zip(1..)
        .find(|&(floor, _)| floor < 0)
        .unwrap()
        .1
}

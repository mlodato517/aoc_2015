

use std::fs::read_to_string;


fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected input"),
        }
    }
    floor
}

fn part2(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected input"),
        }
        if floor < 0 {
            return i + 1;
        }
    }
    panic!("never went negative");
}

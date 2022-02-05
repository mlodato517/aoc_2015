use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("part1: {}", day4::part1(&input));
    println!("part2: {}", day4::part2(&input));
}

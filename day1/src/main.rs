

use std::fs::read_to_string;


fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut floor : i32 = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => panic!("derp"),
        }
    }
    println!("{}", floor);
}



use std::fs::read_to_string;


fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut floor : i32 = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => panic!("derp"),
        }
        if floor < 0 {
            println!("{}", i+1);
            break;
        }
    }
}

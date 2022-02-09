
fn find_md5_starting_with(input: &str, num_zeros: u8) -> u32 {
    let mut i = 1;
    let num_zero_bytes = num_zeros / 2;
    
    'outer: loop {
        let input = format!("{input}{i}");
        let hash: [u8; 16] = md5::compute(&input).into();
        for j in 0..num_zero_bytes {
            if hash[j as usize] != 0 {
                i += 1;
                continue 'outer;
            }
        }
        if (num_zeros & 1) == 1 {
            if (hash[num_zero_bytes as usize] & 0b11110000) != 0 {
                i += 1;
                continue 'outer;
            }
        }
        return i;
    }
}

pub fn part1(input: &str) -> u32 {
    find_md5_starting_with(input, 5)
}

pub fn part2(input: &str) -> u32 {
    find_md5_starting_with(input, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_examples() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}

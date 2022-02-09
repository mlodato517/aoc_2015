use std::fmt::Write;

fn find_md5_starting_with(input: &str, prefix: &str) -> u32 {
    let mut i = 1;
    let mut key = String::with_capacity(input.len() + 11);
    key += input;
    loop {
        key.truncate(input.len());
        write!(&mut key, "{i}");
        let hash = md5::compute(&key);
        let hash = format!("{:X}", &hash);
        if hash.starts_with(prefix) {
            return i;
        }
        i += 1;
    }
}

pub fn part1(input: &str) -> u32 {
    find_md5_starting_with(input, "00000")
}

pub fn part2(input: &str) -> u32 {
    find_md5_starting_with(input, "000000")
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

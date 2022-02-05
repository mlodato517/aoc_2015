fn find_md5_starting_with(input: &str, prefix: &str) -> u32 {
    (1..)
        .find(|i| {
            let input = format!("{input}{i}");
            let hash = md5::compute(&input);
            let hash = format!("{:X}", &hash);
            hash.starts_with(prefix)
        })
        .unwrap()
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

    #[test]
    fn test_chris_part_1_input() {
        assert_eq!(part1(include_str!("../input.txt")), 282749);
    }

    #[test]
    fn test_chris_part_2_input() {
        assert_eq!(part2(include_str!("../input.txt")), 9962624);
    }
}

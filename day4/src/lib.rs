
pub fn part1(input: &str) -> u32 {
    let mut i = 1;
    loop {
        let input = format!("{input}{i}");
        let hash = md5::compute(&input);
        let hash = format!("{:X}", &hash);
        if hash.starts_with("00000") {
            return i;
        }
        i += 1;
    }
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

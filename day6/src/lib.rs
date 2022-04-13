mod common;
mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "turn on 0,0 through 999,999";
        let num_lights_on = part1(input);

        assert_eq!(num_lights_on, 1_000_000);
    }

    #[test]
    fn test_part2() {
        let input = "turn on 0,0 through 999,999";
        let brightness = part2(input);

        assert_eq!(brightness, 1_000_000);

        let input = "toggle 0,0 through 999,999";
        let brightness = part2(input);

        assert_eq!(brightness, 2_000_000);
    }
}

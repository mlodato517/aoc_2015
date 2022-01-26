/// Calculates the wrapping paper required for wrapping the boxes in `input`.
/// Each line of input is one box given as `lxwxh` where `l`, `w`, and `h` are the length, width,
/// and height of the box, respectively. The paper required to wrap each box is:
/// `surface_area_of_box + extra` where `surface_area_of_box = 2lw + 2wh + 2hl` and `extra` is the
/// area of the smallest side of the box.
pub fn part1(input: &str) -> u32 {
    input.lines().map(paper_for_present).sum()
}

/// Calculates the paper required to wrap one present. See [`part1`] for more details.
fn paper_for_present(line: &str) -> u32 {
    let (l, w, h) = dimensions(line);

    let area1 = l * w;
    let area2 = w * h;
    let area3 = h * l;

    let sa = 2 * (area1 + area2 + area3);
    let area_of_smallest_side = [area1, area2, area3].into_iter().min().unwrap();

    sa + area_of_smallest_side
}

/// Calculates the ribbon required for wrapping the boxes in `input`.
/// Each line of input is one box given as `lxwxh` where `l`, `w`, and `h` are the length, width,
/// and height of the box, respectively. The ribbon required to wrap each box is:
/// `smallest_perimeter + extra` where each perimeter is `2(dimension1 + dimension2)` and `extra`
/// is the volume of the box - `l * w * h`.
pub fn part2(input: &str) -> u32 {
    input.lines().map(ribbon_for_present).sum()
}

/// Calculates the ribbon required to wrap one present. See [`part2`] for more details.
fn ribbon_for_present(line: &str) -> u32 {
    let (l, w, h) = dimensions(line);

    let perimeter1 = 2 * l + 2 * w;
    let perimeter2 = 2 * w + 2 * h;
    let perimeter3 = 2 * h + 2 * l;

    let volume = l * w * h;

    let smallest_perimeter = [perimeter1, perimeter2, perimeter3]
        .into_iter()
        .min()
        .unwrap();

    volume + smallest_perimeter
}

/// Parses a line of `lxwxh` into the length, width, and height of the box.
/// Lines are expected to be well formed and parsing invalid lines will panic.
fn dimensions(line: &str) -> (u32, u32, u32) {
    let mut dimensions = line.split('x');

    let l = dimensions.next().unwrap();
    let w = dimensions.next().unwrap();
    let h = dimensions.next().unwrap();

    let l = l.parse::<u32>().unwrap();
    let w = w.parse::<u32>().unwrap();
    let h = h.parse::<u32>().unwrap();

    (l, w, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_examples() {
        assert_eq!(paper_for_present("2x3x4"), 58);
        assert_eq!(paper_for_present("1x1x10"), 43);
    }

    #[test]
    fn part_1_mark_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1598415);
    }

    #[test]
    fn test_part_2_examples() {
        assert_eq!(ribbon_for_present("2x3x4"), 34);
        assert_eq!(ribbon_for_present("1x1x10"), 14);
    }

    #[test]
    fn part_2_mark_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 3812909);
    }
}

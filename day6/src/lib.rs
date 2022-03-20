struct Instruction {
    start: (usize, usize),
    end: (usize, usize),
    operation: Operation,
}
enum Operation {
    TurnOff,
    TurnOn,
    Toggle,
}

pub fn part1(input: &str) -> u32 {
    let mut lights = Box::new([[false; 1000]; 1000]);
    for line in input.lines() {
        let instruction = parse_instruction(line);
        let (x0, y0) = instruction.start;
        let (x1, y1) = instruction.end;
        let operation: fn(bool) -> bool = match instruction.operation {
            Operation::Toggle => |light| !light,
            Operation::TurnOff => |_light| false,
            Operation::TurnOn => |_light| true,
        };
        for row in &mut lights[x0..=x1] {
            for light in &mut row[y0..=y1] {
                *light = operation(*light);
            }
        }
    }
    let mut count = 0;
    for row in lights.iter() {
        for light in row {
            if *light {
                count += 1;
            }
        }
    }
    count
}

fn parse_instruction(line: &str) -> Instruction {
    //  0123456789
    // "turn off" <-- then start at 9
    // "turn on" <--- then start at 8
    // "toggle " <--- then start at 7
    //        ^--- this byte can tell us what we're doing.
    let operation = match line.as_bytes()[6] {
        b'f' => Operation::TurnOff,
        b'n' => Operation::TurnOn,
        b' ' => Operation::Toggle,
        _ => panic!("Invalid thing - we should make this go away"),
    };
    let rest_of_string = match operation {
        Operation::TurnOff => &line[9..],
        Operation::TurnOn => &line[8..],
        Operation::Toggle => &line[7..],
    };

    let mut pieces = rest_of_string.split_ascii_whitespace();

    let first_pair = pieces.next().unwrap();

    // Skip next word - it's "through"
    pieces.next();

    let second_pair = pieces.next().unwrap();

    Instruction {
        start: parse_coordinate(first_pair),
        end: parse_coordinate(second_pair),
        operation,
    }
}

fn parse_coordinate(s: &str) -> (usize, usize) {
    let mut split = s.split(',');
    let x = split.next().unwrap();
    let y = split.next().unwrap();

    (x.parse().unwrap(), y.parse().unwrap())
}

pub fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "turn on 0,0 through 999,999";
        let num_lights_on = part1(input);

        assert_eq!(num_lights_on, 1_000_000);
    }

    #[test]
    fn test_part_2() {
        let input = "turn off 0,0 through 999,999";
        let num_lights_on = part1(input);

        assert_eq!(num_lights_on, 0);
    }
}

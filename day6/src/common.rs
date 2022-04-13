pub(crate) struct Instruction {
    pub(crate) start: (usize, usize),
    pub(crate) end: (usize, usize),
    pub(crate) operation: Operation,
}
pub(crate) enum Operation {
    TurnOff,
    TurnOn,
    Toggle,
}

pub(crate) fn parse_instruction(line: &str) -> Instruction {
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

pub(crate) fn change_lights<Gen, LightChanger, Light>(
    instruction: Instruction,
    lights: &mut [[Light; 1000]],
    light_changer_gen: Gen,
) where
    Gen: Fn(Instruction) -> LightChanger,
    LightChanger: Fn(Light) -> Light,
    Light: Copy,
{
    let (x0, y0) = instruction.start;
    let (x1, y1) = instruction.end;
    let operation = light_changer_gen(instruction);
    for row in &mut lights[x0..=x1] {
        for light in &mut row[y0..=y1] {
            *light = operation(*light);
        }
    }
}

pub(crate) fn parse_coordinate(s: &str) -> (usize, usize) {
    let mut split = s.split(',');
    let x = split.next().unwrap();
    let y = split.next().unwrap();

    (x.parse().unwrap(), y.parse().unwrap())
}

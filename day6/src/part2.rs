use super::common::{change_lights, parse_instruction, Instruction, Operation};

pub fn part2(input: &str) -> usize {
    let mut lights = vec![[0usize; 1000]; 1000];
    for line in input.lines() {
        let instruction = parse_instruction(line);
        change_lights(instruction, &mut lights, light_changer_for_instruction);
    }
    let mut brightness = 0;
    for row in lights.iter() {
        for light in row {
            brightness += light;
        }
    }
    brightness
}

fn light_changer_for_instruction(instruction: Instruction) -> impl Fn(usize) -> usize {
    match instruction.operation {
        Operation::Toggle => |light: usize| light + 2,
        Operation::TurnOff => |light: usize| light.saturating_sub(1),
        Operation::TurnOn => |light: usize| light + 1,
    }
}

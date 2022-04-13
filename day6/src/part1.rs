use super::common::{change_lights, parse_instruction, Instruction, Operation};

pub fn part1(input: &str) -> u32 {
    let mut lights = vec![[false; 1000]; 1000];
    for line in input.lines() {
        let instruction = parse_instruction(line);
        change_lights(instruction, &mut lights, light_changer_for_instruction);
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

fn light_changer_for_instruction(instruction: Instruction) -> impl Fn(bool) -> bool {
    match instruction.operation {
        Operation::Toggle => |light: bool| !light,
        Operation::TurnOff => |_light: bool| false,
        Operation::TurnOn => |_light: bool| true,
    }
}

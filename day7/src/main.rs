use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<_> = input.lines().map(parse).collect();
    dbg!(&input[0]);
}

fn parse(s: &str) -> (InputOrGate, String) {
    let parts: Vec<_> = s.split(" -> ").collect();
    assert!(parts.len() == 2);
    let output_wire = parts[1];
    let lhs_parts: Vec<_> = parts[0].split(' ').collect();
    let lhs = match lhs_parts.len() {
        1 => InputOrGate::Input(lhs_parts[0].parse().unwrap()),
        2 => {
            assert_eq!("NOT", lhs_parts[0]);
            let input: Input = lhs_parts[1].parse().unwrap();
            let gate = Gate::Not(input);
            InputOrGate::Gate(gate)
        }
        3 => {
            let l_operand: Input = lhs_parts[0].parse().unwrap();
            let r_operand: Input = lhs_parts[2].parse().unwrap();
            let gate: Gate = match lhs_parts[1] {
                "AND" => Gate::And(l_operand, r_operand),
                "OR" => Gate::Or(l_operand, r_operand),
                "LSHIFT" => Gate::Lshift(l_operand, r_operand),
                "RSHIFT" => Gate::Rshift(l_operand, r_operand),
                _ => panic!("Bad gate type"),
            };
            InputOrGate::Gate(gate)
        }
        _ => panic!("bad input with too many components"),
    };
    (lhs, output_wire.to_owned())
}

#[derive(Debug)]
enum Input {
    Signal(u16),
    Wire(String),
}

impl FromStr for Input {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse().map_or_else(|_| Self::Wire(s.to_owned()), |v| Self::Signal(v)))
    }
}

#[derive(Debug)]
enum InputOrGate {
    Input(Input),
    Gate(Gate),
}

#[derive(Debug)]
enum Gate {
    And(Input, Input),
    Lshift(Input, Input),
    Not(Input),
    Rshift(Input, Input),
    Or(Input, Input),
}
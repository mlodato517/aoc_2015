use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<_> = input.lines().map(parse).collect();
    let mut circuit = Circuit::new(input);

    const TARGET_WIRE: &str = "a";
    let a_value = circuit.wire_value(TARGET_WIRE);
    println!("Value of 'a' wire is {a_value}!");
}

struct Circuit {
    inputs: HashMap<String, InputOrGate>,
    cache: HashMap<String, u16>,
}

impl Circuit {
    fn new(input: Vec<(InputOrGate, String)>) -> Self {
        let inputs: HashMap<_, _> = input
            .into_iter()
            .map(|(input_or_gate, wire_name)| (wire_name, input_or_gate))
            .collect();
        let cache = HashMap::new();

        Self { inputs, cache }
    }

    fn wire_value(&mut self, wire_name: &str) -> u16 {
        if let Some(value) = self.cache.get(wire_name) {
            return *value;
        }

        let (name, input_or_gate) = self.inputs.remove_entry(wire_name).unwrap();
        let value = match input_or_gate {
            InputOrGate::Input(input) => self.resolve(&input),
            InputOrGate::Gate(Gate::And(lhs, rhs)) => self.resolve(&lhs) & self.resolve(&rhs),
            InputOrGate::Gate(Gate::Lshift(lhs, rhs)) => self.resolve(&lhs) << self.resolve(&rhs),
            InputOrGate::Gate(Gate::Not(input)) => !self.resolve(&input),
            InputOrGate::Gate(Gate::Rshift(lhs, rhs)) => self.resolve(&lhs) >> self.resolve(&rhs),
            InputOrGate::Gate(Gate::Or(lhs, rhs)) => self.resolve(&lhs) | self.resolve(&rhs),
        };

        self.cache.insert(name, value);
        value
    }

    fn resolve(&mut self, input: &Input) -> u16 {
        match input {
            Input::Signal(signal) => *signal,
            Input::Wire(source_wire) => self.wire_value(source_wire),
        }
    }
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
        Ok(s.parse()
            .map_or_else(|_| Self::Wire(s.to_owned()), Self::Signal))
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


use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<_> = input.lines().map(parse).collect();
    let mut circuit = Circuit::new(input);

    const TARGET_WIRE: &str = "a";
    let a_value = circuit.wire_value(TARGET_WIRE);
    println!("Value of 'a' wire is originally {a_value}!");

    circuit.update_wire("b", a_value);
    let a_value = circuit.wire_value(TARGET_WIRE);
    println!("Value of 'a' wire is updated to {a_value}!");
}

struct CircuitElement {
    input: InputOrGate,
    output: Option<u16>,
}

impl CircuitElement {
    fn new(input: InputOrGate) -> Self {
        Self {
            input,
            output: None,
        }
    }
}

struct Circuit {
    inputs: HashMap<String, CircuitElement>,
}

impl Circuit {
    fn new(input: Vec<(InputOrGate, String)>) -> Self {
        let inputs: HashMap<_, _> = input
            .into_iter()
            .map(|(input_or_gate, wire_name)| (wire_name, CircuitElement::new(input_or_gate)))
            .collect();

        Self { inputs }
    }

    fn wire_value(&mut self, wire_name: &str) -> u16 {
        if let Some(output) = self
            .inputs
            .get(wire_name)
            .and_then(|element| element.output)
        {
            return output;
        }

        let (name, mut input_or_gate) = self.inputs.remove_entry(wire_name).unwrap();
        let output = match &input_or_gate.input {
            InputOrGate::Input(input) => self.resolve(input),
            InputOrGate::Gate(Gate::And(lhs, rhs)) => self.resolve(lhs) & self.resolve(rhs),
            InputOrGate::Gate(Gate::Lshift(lhs, rhs)) => self.resolve(lhs) << self.resolve(rhs),
            InputOrGate::Gate(Gate::Not(input)) => !self.resolve(input),
            InputOrGate::Gate(Gate::Rshift(lhs, rhs)) => self.resolve(lhs) >> self.resolve(rhs),
            InputOrGate::Gate(Gate::Or(lhs, rhs)) => self.resolve(lhs) | self.resolve(rhs),
        };
        input_or_gate.output = Some(output);

        self.inputs.insert(name, input_or_gate);
        output
    }

    fn update_wire(&mut self, wire_name: &str, wire_value: u16) {
        self.inputs.get_mut(wire_name).unwrap().input =
            InputOrGate::Input(Input::Signal(wire_value));

        // The circuit has changed - any outputs could now be different!
        for element in self.inputs.values_mut() {
            element.output = None;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chris_input() {
        let input = include_str!("../input.txt");
        let input: Vec<_> = input.lines().map(parse).collect();
        let mut circuit = Circuit::new(input);

        const TARGET_WIRE: &str = "a";
        let a_value = circuit.wire_value(TARGET_WIRE);
        assert_eq!(a_value, 46065);

        circuit.update_wire("b", a_value);
        let a_value = circuit.wire_value(TARGET_WIRE);
        assert_eq!(a_value, 14134);
    }
}

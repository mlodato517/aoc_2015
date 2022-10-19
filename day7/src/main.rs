use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.lines().map(parse);
    let mut circuit = Circuit::new(input);

    const TARGET_WIRE: &str = "a";
    let a_value = circuit.wire_value(TARGET_WIRE);
    println!("Value of 'a' wire is originally {a_value}!");

    circuit.update_wire("b", a_value);
    let a_value = circuit.wire_value(TARGET_WIRE);
    println!("Value of 'a' wire is updated to {a_value}!");
}

struct CircuitElement<'a> {
    input: InputOrGate<'a>,
    output: Option<u16>,
}

impl<'a> CircuitElement<'a> {
    fn new(input: InputOrGate<'a>) -> Self {
        Self {
            input,
            output: None,
        }
    }
}

struct Circuit<'a> {
    inputs: HashMap<&'a str, CircuitElement<'a>>,
}

impl<'a> Circuit<'a> {
    fn new<I>(input: I) -> Self
    where
        I: IntoIterator<Item = (InputOrGate<'a>, &'a str)>,
    {
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

fn parse(s: &str) -> (InputOrGate, &str) {
    let mut parts = s.split(" -> ");

    let lhs = parts.next().expect("Line is empty!");
    let output_wire = parts.next().expect("Line missing right hand side!");
    assert!(parts.next().is_none());

    let mut lhs_parts = lhs.split(' ');
    let lhs = match (lhs_parts.next(), lhs_parts.next(), lhs_parts.next()) {
        (Some(signal), None, None) => InputOrGate::Input(Input::parse(signal)),
        (Some("NOT"), Some(input), None) => {
            let input = Input::parse(input);
            let gate = Gate::Not(input);
            InputOrGate::Gate(gate)
        }
        (Some(input1), Some(operation), Some(input2)) => {
            let l_operand = Input::parse(input1);
            let r_operand = Input::parse(input2);
            let gate: Gate = match operation {
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
    (lhs, output_wire)
}

#[derive(Debug)]
enum Input<'a> {
    Signal(u16),
    Wire(&'a str),
}

impl<'a> Input<'a> {
    fn parse(s: &'a str) -> Self {
        s.parse().map_or_else(|_| Self::Wire(s), Self::Signal)
    }
}

#[derive(Debug)]
enum InputOrGate<'a> {
    Input(Input<'a>),
    Gate(Gate<'a>),
}

#[derive(Debug)]
enum Gate<'a> {
    And(Input<'a>, Input<'a>),
    Lshift(Input<'a>, Input<'a>),
    Not(Input<'a>),
    Rshift(Input<'a>, Input<'a>),
    Or(Input<'a>, Input<'a>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chris_input() {
        let input = include_str!("../input.txt");
        let input = input.lines().map(parse);
        let mut circuit = Circuit::new(input);

        const TARGET_WIRE: &str = "a";
        let a_value = circuit.wire_value(TARGET_WIRE);
        assert_eq!(a_value, 46065);

        circuit.update_wire("b", a_value);
        let a_value = circuit.wire_value(TARGET_WIRE);
        assert_eq!(a_value, 14134);
    }
}

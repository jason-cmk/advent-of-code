use std::{collections::HashMap, fmt::Display, borrow::BorrowMut};

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Command::Assign => "Assign",
            Command::AND => "AND",
            Command::OR => "OR",
            Command::LSHIFT => "LSHIFT",
            Command::RSHIFT => "RSHIFT",
            Command::NOT => "NOT",
        };

        write!(f, "{result}")
    }
}

enum Command {
    Assign,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
}

fn get_command(line: &str) -> Command {
    if (*line).contains("AND") {
        Command::AND
    } else if (*line).contains("OR") {
        Command::OR
    } else if (*line).contains("LSHIFT") {
        Command::LSHIFT
    } else if (*line).contains("RSHIFT") {
        Command::RSHIFT
    } else if (*line).contains("NOT") {
        Command::NOT
    } else {
        Command::Assign
    }
}

fn get_args(line: &str) -> (&str, &str, &str) {
    let mut result: Vec<&str> = Vec::new();

    for word in line.split(" ") {
        match word {
            "->" | "AND" | "OR" | "LSHIFT" | "RSHIFT" | "NOT" => (),
            _ => result.push(word),
        }
    }

    let mut result_iter = result.iter();

    match result.len() {
        2 => (result_iter.next().unwrap(), "", result_iter.next().unwrap()),
        3 => (
            result_iter.next().unwrap(),
            result_iter.next().unwrap(),
            result_iter.next().unwrap(),
        ),
        _ => panic!("hmm that's not the right number of args"),
    }
}

struct Wire {
    address: String,
    value: Option<u16>,
    instruction: String,
}

struct Circuit {
    board: HashMap<String, Wire>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {
            board: HashMap::new(),
        }
    }

    fn add_wire(&mut self, line: &str) -> () {
        let address = line.split(" ").last().unwrap();

        let wire = Wire {
            address: address.to_owned(),
            value: Option::None,
            instruction: line.to_owned(),
        };

        self.board.insert(address.to_owned(), wire);
    }

    fn get_value(&mut self, address: &str) -> u16 {
        let wire = self.board.get_mut(address).unwrap();

        let arg1 = self.get_value(get_args(&wire.instruction).0);

        if wire.value.is_some() {
            return wire.value.unwrap();
        }

        let command = get_command(&wire.instruction);

        let args = get_args(&wire.instruction);

        let value = match command {
            Command::Assign => args.0.parse::<u16>().unwrap(),
            Command::AND => self.get_value(args.0) & self.get_value(args.1),
            Command::OR => self.get_value(args.0) | self.get_value(args.1),
            Command::LSHIFT => self.get_value(args.0) << self.get_value(args.1),
            Command::RSHIFT => todo!(),
            Command::NOT => todo!(),
        };

        let mutable_wire = self.board.get_mut(address).unwrap();
        mutable_wire.value = Some(value);

        return value;
    }
}

fn main() {
    let file_contents = include_str!("input.txt").trim();

    let mut circuit = Circuit::new();

    for line in file_contents.lines() {
        circuit.add_wire(line);
    }

    let a_value = circuit.get_value("d");
    print!("part 1: {a_value}");
}

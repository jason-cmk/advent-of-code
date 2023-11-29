use std::{collections::HashMap, fmt::Display};

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

struct StateMachine {
    state: HashMap<String, u16>,
}

#[derive(Debug)]
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
        3 => (result_iter.next().unwrap(), result_iter.next().unwrap(), result_iter.next().unwrap()),
        _ => panic!("hmm that's not the right number of args")
    }
}

impl StateMachine {
    fn new() -> StateMachine {
        StateMachine {
            state: HashMap::new(),
        }
    }

    fn compute(&mut self, line: &str) -> () {
        let command = get_command(line);

        let args = get_args(line);

        let destination = args.2;

        if !self.state.contains_key(destination) {
            self.state.insert(destination.to_owned(), 0u16);
        }

        if !self.state.contains_key(args.0) {
            self.state.insert(args.0.to_owned(), 0u16);
        }

        if !self.state.contains_key(args.1) {
            self.state.insert(args.1.to_owned(), 0u16);
        }

        let arg0 = self.state[args.0];
        let arg1 = self.state[args.1];
        print!("{command} {} ({}) {} ({}) to {destination}", args.0, arg0, args.1, arg1);

        let result = match command {
            Command::Assign => self.state[args.0],
            Command::AND => self.state[args.0] & self.state[args.1],
            Command::OR => self.state[args.0] | self.state[args.1],
            Command::LSHIFT => self.state[args.0] << args.1.parse::<u16>().unwrap(),
            Command::RSHIFT => self.state[args.0] >> args.1.parse::<u16>().unwrap(),
            Command::NOT => !self.state[args.0]
        };


        self.state.insert(destination.to_owned(), result);
        let output = self.state[destination];
        println!(" => {output}");
    }
}

fn main() {
    let file_contents = include_str!("input.txt").trim();

    let mut state_machine = StateMachine::new();

    for line in file_contents.lines() {
        state_machine.compute(line);
    }

//     for key in state_machine.state.keys() {
//         let value = state_machine.state[key];
//         println!("{key}: {value}");
//     }

    println!("part 1: {}", state_machine.state["a"]);
}

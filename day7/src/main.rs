use std::collections::HashMap;

struct StateMachine {
    state: HashMap<String, u16>,
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
    }
    else if (*line).contains("OR") {
        Command::OR
    }
    else if (*line).contains("LSHIFT") {
        Command::LSHIFT
    }
    else if (*line).contains("RSHIFT") {
        Command::RSHIFT
    }
    else if (*line).contains("NOT") {
        Command::NOT
    }
    else {
        Command::Assign
    }
}

impl StateMachine {
    fn new() -> StateMachine {
        StateMachine {
            state: HashMap::new(),
        }
    }

    fn compute(&self, line: &str) -> () {
        let command = get_command(line);
    }
}

fn main() {
    let file_contents = include_str!("test.txt").trim();

    let mut stateMachine = StateMachine::new();

    for line in file_contents.lines() {
        stateMachine.compute(line);
    }
}

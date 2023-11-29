use regex::Regex;

struct Lights {
    grid: Vec<Vec<bool>>,
    grid_2: Vec<Vec<isize>>
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

fn get_action(line: &str) -> Action {
    if line.split(" ").next().unwrap() == "toggle" {
        return Action::Toggle;
    }

    let on_or_off = line.split(" ").skip(1).next().unwrap();

    match on_or_off {
        "on" => Action::TurnOn,
        "off" => Action::TurnOff,
        _ => panic!("could not map to action!"),
    }
}

fn get_positions(line: &str) -> ((usize, usize), (usize, usize)) {
    let re = Regex::new(r"^.*?(\d+,\d+).*?(\d+,\d+)$").unwrap();

    let (_, [left_str, right_str]) = re.captures(line).unwrap().extract();

    let mut left = left_str.split(",").map(|x| x.parse::<usize>().unwrap());
    let left_tuple: (usize, usize) = (left.next().unwrap(), left.next().unwrap());

    let mut right = right_str.split(",").map(|x| x.parse::<usize>().unwrap());
    let right_tuple: (usize, usize) = (right.next().unwrap(), right.next().unwrap());

    (left_tuple, right_tuple)
}

impl Lights {
    fn new() -> Lights {
        let mut lights = Lights {
            grid: vec![],
            grid_2: vec![]
        };

        for i in 0..1000 {
            lights.grid.push(vec![]);
            lights.grid_2.push(vec![]);
            for _ in 0..1000 {
                lights.grid[i].push(false);
                lights.grid_2[i].push(0);
            }
        }

        return lights;
    }

    fn instruct(&mut self, instruction: &str) {
        let action = get_action(instruction);

        let (start, end) = get_positions(instruction);

        for row in start.0..end.0+1 {
            for col in start.1..end.1+1 {
                self.grid[row][col] = match action {
                    Action::TurnOn => true,
                    Action::TurnOff => false,
                    Action::Toggle => !self.grid[row][col],
                };

                self.grid_2[row][col] = match action {
                    Action::TurnOn => self.grid_2[row][col] + 1,
                    Action::TurnOff => 0.max(self.grid_2[row][col] - 1),
                    Action::Toggle => self.grid_2[row][col] + 2,
                };
            }
        }
    }

    fn count_lights(&self) -> usize {
        return self.grid.iter().flatten().filter(|x| **x == true).count();
    }

    fn count_lights_2(&self) -> isize {
        return self.grid_2.iter().flatten().map(|x| *x).sum();
    }
}

fn main() {
    let file_contents = include_str!("input.txt.");

    let mut lights = Lights::new();

    for line in file_contents.lines() {
        lights.instruct(line);
    }

    let lights_on: usize = lights.count_lights();
    let lights_on_2: isize = lights.count_lights_2();

    println!("part 1 {lights_on}");
    println!("part 2 {lights_on_2}");
}

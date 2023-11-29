use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Location
{
    x: i32,
    y: i32
}

struct Map {
    map: HashSet<Location>
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Map {
    fn new(houses: &str) -> Map {
        let mut hash_map: HashSet<Location> = HashSet::new();
        
        let mut current_location = Location { x: 0, y: 0 };
        let mut robo_current_location = Location { x: 0, y: 0 };

        hash_map.insert(current_location);
        
        for (i, c) in houses.chars().enumerate() {
            let direction = get_direction(c);

            if i % 2 == 0 {
                let next_location = get_next_location(current_location, direction);
                hash_map.insert(next_location);

                current_location = next_location;
            } else {
                let next_location = get_next_location(robo_current_location, direction);
                hash_map.insert(next_location);

                robo_current_location = next_location;
            }
        }

        Map {
            map: hash_map
        }
    }

    fn houses_visited(&self) -> usize {
        return self.map.len();
    }
}

fn get_direction(c: char) -> Direction {
    match c {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("direction could not be mapped!")
    }
}

fn get_next_location(current_location: Location, direction: Direction) -> Location {
    match direction {
        Direction::Up => Location { x: current_location.x, y: current_location.y + 1 },
        Direction::Down => Location { x: current_location.x, y: current_location.y - 1 },
        Direction::Left => Location { x: current_location.x - 1, y: current_location.y },
        Direction::Right => Location { x: current_location.x + 1, y: current_location.y },
    }
}

fn main() {
    let file_contents = include_str!("day3.txt");
    let map = Map::new(file_contents.trim());

    println!("houses visited: {}", map.houses_visited());
}

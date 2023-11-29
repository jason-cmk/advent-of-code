use std::str::FromStr;

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(sizespec: &str) -> Present {
        let mut sizes = sizespec.split('x').map(|s| u32::from_str(s).unwrap());
        Present {
            length: sizes.next().unwrap(),
            width: sizes.next().unwrap(),
            height: sizes.next().unwrap(),
        }
    }

    fn surface_area(&self) -> u32 {
        return self.sides().iter().map(|x| x * 2).sum::<u32>() + self.slack();
    }

    fn sides(&self) -> Vec<u32> {
        return vec![
            self.length * self.width,
            self.length * self.height,
            self.width * self.height,
        ];
    }

    fn slack(&self) -> u32 {
        return *self.sides().iter().min().unwrap();
    }

    fn lengths(&self) -> Vec<u32> {
        return vec![self.length, self.width, self.height];
    }

    fn ribbon_length(&self) -> u32 {
        let mut sides = self.lengths();
        sides.sort();

        let ribbon_sides_length: u32 = sides.iter().take(2).sum();
        let total_sides_length = ribbon_sides_length * 2;

        let cube = sides.into_iter().reduce(|x, r| x * r).unwrap();

        return total_sides_length + cube;
    }
}

struct Presents {
    presents: Vec<Present>,
}

impl Presents {
    fn new(sizespecs: &str) -> Presents {
        Presents {
            presents: sizespecs.lines().map(|x| Present::new(x)).collect(),
        }
    }

    fn total_surface_area(&self) -> u32 {
        return self.presents.iter().map(|x| x.surface_area()).sum();
    }

    fn total_ribbon_length(&self) -> u32 {
        return self.presents.iter().map(|x| x.ribbon_length()).sum();
    }
}

fn main() {
    let presents = Presents::new(include_str!("day2.txt"));

    println!(
        "total surface area: {}",
        presents.total_surface_area().to_string()
    );
    println!(
        "total ribbon length: {}",
        presents.total_ribbon_length().to_string()
    );
}

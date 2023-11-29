fn main() {
    let file_contents = include_str!("input.txt").trim();
    
    let mut counter: isize = 0;
    let mut position: usize = 0;
    let mut part_2_ans: usize = 0;
    let mut part_2_found = false;

    for c in file_contents.chars() {
        if counter < 0 && !part_2_found {
            part_2_ans = position;
            part_2_found = true;
        }

        counter += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("oh fuck")
        };

        position += 1;
    }

    println!("part 1: {counter}");
    println!("part 2: {part_2_ans}");
}

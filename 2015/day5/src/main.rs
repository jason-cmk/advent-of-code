use collection_macros::hashset;
use std::collections::HashSet;

trait NiceOMeter {
    fn is_nice(&self) -> bool;

    fn is_nice_2_pairs(&self) -> bool;

    fn is_nice_2_between(&self) -> bool;
}

impl NiceOMeter for str {
    fn is_nice(&self) -> bool {
        let vowels: HashSet<char> = hashset!['a', 'e', 'i', 'o', 'u'];
        let invalid_pairs: HashSet<(char, char)> =
            hashset![('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
        let mut dupes_found = false;

        let mut previous_char = self.chars().next().unwrap();

        let mut vowel_count = 0u32;

        if vowels.contains(&previous_char) {
            vowel_count += 1;
        }

        let mut is_invalid_pair = false;

        for c in self.chars().skip(1) {
            let current_char = c;

            if vowels.contains(&current_char) {
                vowel_count += 1;
            }

            if !is_invalid_pair {
                is_invalid_pair = invalid_pairs.contains(&(previous_char, current_char));
            }

            if !dupes_found {
                if previous_char == current_char {
                    dupes_found = true;
                }
            }

            previous_char = current_char;
        }

        let vowel_count_met = vowel_count >= 3;

        return vowel_count_met && !is_invalid_pair && dupes_found;
    }

    fn is_nice_2_pairs(&self) -> bool {
        let mut previous_char = self.chars().next().unwrap();

        for c in self.chars().skip(1) {
            let current_char = c;

            let pair = format!("{previous_char}{current_char}");

            let left = self.find(&pair).unwrap();
            let right = self.rfind(&pair).unwrap();

            if right - left >= 2 {
                return true;
            }

            previous_char = current_char;
        }
        
        return false;
    }

    fn is_nice_2_between(&self) -> bool {
        let mut char_iter = self.chars();

        let mut tail = char_iter.next().unwrap();
        let mut middle = char_iter.next().unwrap();

        for c in char_iter {
            let head = c;

            if head == tail {
                return true;
            }

            tail = middle;
            middle = head;
        }

        return false;
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let mut nice_count: u32 = 0;
    let mut nice_count_2: u32 = 0;

    for line in input_string.split("\n").filter(|x| x.len() > 0) {
        if line.is_nice() {
            nice_count += 1;
        }

        if line.is_nice_2_pairs() && line.is_nice_2_between() {
            println!("nice: {line}");
            nice_count_2 += 1;
        } else {
            println!("naughty: {line}");
        }
    }

    println!("part 1: {nice_count}");
    println!("part 2: {nice_count_2}");
}

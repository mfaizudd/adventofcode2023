use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("input.txt").expect("Invalid input");
    let mut sum = 0;
    for line in content.lines() {
        let colon_index = line.find(':').expect("No colon found");
        let mut chars = line.chars();
        chars.nth(colon_index);
        let mut digits = String::new();
        let mut winning = HashSet::new();
        let mut parsing_winning = true;
        let mut points = 0;
        while let Some(ch) = chars.next() {
            match ch {
                '0'..='9' => digits.push(ch),
                ' ' if !digits.is_empty() => {
                    let val = digits.parse::<i32>().expect("Invalid digits");
                    if parsing_winning {
                        winning.insert(val);
                    } else if winning.contains(&val) {
                        if points == 0 {
                            points = 1;
                        } else {
                            points <<= 1;
                        }
                    }
                    digits.clear();
                }
                '|' => parsing_winning = false,
                _ => continue,
            }
        }
        if !digits.is_empty() {
            let val = digits.parse::<i32>().expect("Invalid digits");
            if parsing_winning {
                winning.insert(val);
            } else if winning.contains(&val) {
                if points == 0 {
                    points = 1;
                } else {
                    points <<= 1;
                }
            }
        }
        sum += points;
    }
    println!("Sum: {sum}");
}

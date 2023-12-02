use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("input.txt").expect("Invalid input file");
    let mut sum = 0;
    let mut power = 0;
    for line in content.lines() {
        let mut chars = line.chars();
        let colon_index = line.find(':').expect("No colon?!");
        let id = line[5..colon_index].parse::<i64>().expect("Invalid id?");
        let mut set = get_empty_sets();
        let mut max_set = get_empty_sets();
        let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let mut digits = String::new();
        chars.nth(colon_index + 1);
        let mut possible = true;
        while let Some(ch) = chars.next() {
            match ch {
                ' ' => continue,
                '0'..='9' => {
                    digits.push(ch);
                }
                'r' => {
                    let val = digits.parse::<i32>().expect("Invalid digits");
                    set.entry("red").and_modify(|v| *v = val);
                    if val > max_set["red"] {
                        max_set.entry("red").and_modify(|v| *v = val);
                    }
                    chars.nth(1);
                    digits.clear();
                }
                'g' => {
                    let val = digits.parse::<i32>().expect("Invalid digits");
                    set.entry("green").and_modify(|v| *v = val);
                    if val > max_set["green"] {
                        max_set.entry("green").and_modify(|v| *v = val);
                    }
                    chars.nth(3);
                    digits.clear();
                }
                'b' => {
                    let val = digits.parse::<i32>().expect("Invalid digits");
                    set.entry("blue").and_modify(|v| *v = val);
                    if val > max_set["blue"] {
                        max_set.entry("blue").and_modify(|v| *v = val);
                    }
                    chars.nth(2);
                    digits.clear();
                }
                ';' => {
                    if set["red"] > bag["red"]
                        || set["green"] > bag["green"]
                        || set["blue"] > bag["blue"]
                    {
                        possible = false;
                    }
                    set = get_empty_sets();
                }
                _ => continue,
            }

            // this will skip the addition to the sum
            if set["red"] > bag["red"] || set["green"] > bag["green"] || set["blue"] > bag["blue"] {
                possible = false;
            }
        }
        if possible {
            sum += id;
        }
        power += max_set["red"] * max_set["green"] * max_set["blue"];
    }
    println!("Possible game sum: {}", sum);
    println!("Sum of power: {}", power);
}

fn get_empty_sets<'a>() -> HashMap<&'a str, i32> {
    HashMap::from([("red", 0), ("green", 0), ("blue", 0)])
}

use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut lines = content.lines();
    let times = read_numbers(lines.next().unwrap());
    let distances = read_numbers(lines.next().unwrap());
    let mut result = 0;
    for (i, time) in times.iter().enumerate() {
        let distance = distances.get(i).unwrap();
        let mut min = 0;
        for speed in 0..*time {
            let remainder = time - speed;
            let output = speed * remainder;
            if output > *distance {
                min = speed;
                break;
            }
        }
        let mut max = 0;
        for speed in (1..*time).rev() {
            let remainder = time - speed;
            let output = speed * remainder;
            if output > *distance {
                max = speed;
                break;
            }
        }
        let total = max - min + 1;
        // println!("{total} ({min} - {max})");
        result = if result == 0 { total } else { result * total }
    }
    println!("{result}");
}

fn read_numbers(input: &str) -> Vec<i64> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut times = Vec::new();
    let mut pos = 0;
    while let Some(ch) = chars.get(pos) {
        match ch {
            '0'..='9' => times.push(read_number(&chars, &mut pos)),
            _ => pos += 1,
        }
    }
    times
}

fn read_number(chars: &Vec<char>, pos: &mut usize) -> i64 {
    let mut digits = String::new();
    while let Some(ch) = chars.get(*pos) {
        digits.push(*ch);
        *pos += 1;
        while let Some(ch) = chars.get(*pos) {
            if ch.is_whitespace() {
                *pos += 1;
            } else {
                break;
            }
        }
    }
    digits.parse::<i64>().unwrap()
}

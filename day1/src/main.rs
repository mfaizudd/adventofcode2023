use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("invalid input");
    let mut sum = 0;
    for line in content.lines() {
        let mut result = 0;
        for pos in 0..line.len() {
            if let Some(digit) = extract_digit(line, pos) {
                result = digit * 10;
                break;
            }
        }
        for pos in (0..line.len()).rev() {
            if let Some(digit) = extract_digit(line, pos) {
                result += digit;
                break;
            }
        }
        sum += result;
    }
    println!("{}", sum);
}

fn extract_digit(input: &str, pos: usize) -> Option<u32> {
    let digit = input.chars().nth(pos)?;
    let length = input.len();
    match digit {
        '0'..='9' => digit.to_digit(10),
        'o' if length >= pos + 3 && &input[pos..pos + 3] == "one" => Some(1),
        't' if length >= pos + 3 && &input[pos..pos + 3] == "two" => Some(2),
        't' if length >= pos + 5 && &input[pos..pos + 5] == "three" => Some(3),
        'f' if length >= pos + 4 && &input[pos..pos + 4] == "four" => Some(4),
        'f' if length >= pos + 4 && &input[pos..pos + 4] == "five" => Some(5),
        's' if length >= pos + 3 && &input[pos..pos + 3] == "six" => Some(6),
        's' if length >= pos + 5 && &input[pos..pos + 5] == "seven" => Some(7),
        'e' if length >= pos + 5 && &input[pos..pos + 5] == "eight" => Some(8),
        'n' if length >= pos + 4 && &input[pos..pos + 4] == "nine" => Some(9),
        'e' if pos >= 2 && &input[pos - 2..=pos] == "one" => Some(1),
        'o' if pos >= 2 && &input[pos - 2..=pos] == "two" => Some(2),
        'e' if pos >= 4 && &input[pos - 4..=pos] == "three" => Some(3),
        'r' if pos >= 3 && &input[pos - 3..=pos] == "four" => Some(4),
        'e' if pos >= 3 && &input[pos - 3..=pos] == "five" => Some(5),
        'x' if pos >= 2 && &input[pos - 2..=pos] == "six" => Some(6),
        'n' if pos >= 4 && &input[pos - 4..=pos] == "seven" => Some(7),
        't' if pos >= 4 && &input[pos - 4..=pos] == "eight" => Some(8),
        'e' if pos >= 3 && &input[pos - 3..=pos] == "nine" => Some(9),
        _ => None,
    }
}

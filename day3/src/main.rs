use std::fs;

enum Token {
    Digit,
    Dot,
    Symbol,
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Invalid input");
    let lines: Vec<&str> = content.lines().collect();
    let mut prev: Option<Vec<char>> = None;
    let mut idx = 0;
    let mut next: Option<Vec<char>> = Some(lines[idx].chars().collect());
    let mut sum = 0;
    while let Some(chars) = next {
        next = if idx + 1 >= lines.len() {
            None
        } else {
            Some(lines[idx + 1].chars().collect())
        };
        let mut digits = String::new();
        let mut prev_token = None;
        let mut adjacent_to_symbol = false;
        for char_idx in 0..chars.len() {
            let ch = chars[char_idx];
            match ch {
                '0'..='9' => {
                    digits.push(ch);
                    if let Some(Token::Symbol) = &prev_token {
                        adjacent_to_symbol = true;
                    }
                    if let Some(prev) = &prev {
                        if is_symbol(&prev[char_idx])
                            || (char_idx > 0 && is_symbol(&prev[char_idx - 1]))
                            || (char_idx + 1 < prev.len() && is_symbol(&prev[char_idx + 1]))
                        {
                            adjacent_to_symbol = true;
                        }
                    }
                    if let Some(next) = &next {
                        if is_symbol(&next[char_idx])
                            || (char_idx > 0 && is_symbol(&next[char_idx - 1]))
                            || (char_idx + 1 < next.len() && is_symbol(&next[char_idx + 1]))
                        {
                            adjacent_to_symbol = true;
                        }
                    }
                    prev_token = Some(Token::Digit);
                }
                '.' => {
                    if adjacent_to_symbol
                        && prev_token
                            .as_ref()
                            .is_some_and(|t| matches!(t, Token::Digit))
                    {
                        let val = digits.parse::<u64>().expect("Invalid digits");
                        sum += val;
                    }
                    prev_token = Some(Token::Dot);
                    adjacent_to_symbol = false;
                    digits.clear();
                }
                '!'..='/' | ':'..='@' => {
                    if let Some(Token::Digit) = &prev_token {
                        let val = digits.parse::<u64>().expect("Invalid digits");
                        sum += val;
                        digits.clear();
                    }
                    prev_token = Some(Token::Symbol);
                    adjacent_to_symbol = false;
                }
                _ => continue,
            }
        }
        if adjacent_to_symbol
            && prev_token
                .as_ref()
                .is_some_and(|t| matches!(t, Token::Digit))
        {
            let val = digits.parse::<u64>().expect("Invalid digits");
            sum += val;
            digits.clear();
        }
        prev = Some(chars);
        idx += 1;
    }
    println!("Sum: {}", sum);
}

fn is_symbol(ch: &char) -> bool {
    matches!(ch, '!'..='/' | ':'..='@') && !matches!(ch, '.')
}

use std::{collections::HashMap, fs};

enum Token {
    Digit,
    Dot,
    Symbol(char),
}

struct Symbol {
    value: char,
    row: usize,
    col: usize,
}

impl Symbol {
    fn new(value: char, row: usize, col: usize) -> Symbol {
        Symbol { value, row, col }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Invalid input");
    let lines: Vec<&str> = content.lines().collect();
    let mut prev: Option<Vec<char>> = None;
    let mut idx = 0;
    let mut next: Option<Vec<char>> = Some(lines[idx].chars().collect());
    let mut sum = 0;
    let mut gear_ratios: HashMap<(usize, usize), (u64, u64)> = HashMap::new();
    while let Some(chars) = next {
        next = if idx + 1 >= lines.len() {
            None
        } else {
            Some(lines[idx + 1].chars().collect())
        };
        let mut digits = String::new();
        let mut prev_token = None;
        let mut adjacent_symbol: Option<Symbol> = None;
        for char_idx in 0..chars.len() {
            let ch = chars[char_idx];
            match ch {
                '0'..='9' => {
                    digits.push(ch);
                    if let Some(Token::Symbol(ch)) = prev_token {
                        adjacent_symbol = Some(Symbol::new(ch, idx, char_idx - 1));
                    }
                    if let Some((ch, id)) = prev.as_ref().and_then(|p| extract_symbol(&p, char_idx))
                    {
                        if ch == '*' {
                            gear_ratios.entry((idx - 1, id)).or_insert((0, 0));
                        }
                        adjacent_symbol = Some(Symbol::new(ch, idx - 1, id));
                    }
                    if let Some((ch, id)) = next.as_ref().and_then(|p| extract_symbol(p, char_idx))
                    {
                        if ch == '*' {
                            gear_ratios.entry((idx + 1, id)).or_insert((0, 0));
                        }
                        adjacent_symbol = Some(Symbol::new(ch, idx + 1, id));
                    }
                    prev_token = Some(Token::Digit);
                }
                '.' => {
                    if adjacent_symbol.is_some()
                        && prev_token
                            .as_ref()
                            .is_some_and(|t| matches!(t, Token::Digit))
                    {
                        let val = digits.parse::<u64>().expect("Invalid digits");
                        sum += val;
                        let symbol = adjacent_symbol.unwrap();
                        if symbol.value == '*' {
                            gear_ratios
                                .entry((symbol.row, symbol.col))
                                .and_modify(|r| {
                                    if r.1 == 0 {
                                        r.0 = val;
                                        r.1 = 1;
                                    } else if r.1 == 1 {
                                        r.0 *= val;
                                        r.1 = 2;
                                    } else {
                                        return;
                                    }
                                })
                                .or_insert((val, 1));
                        }
                    }
                    prev_token = Some(Token::Dot);
                    adjacent_symbol = None;
                    digits.clear();
                }
                '!'..='/' | ':'..='@' => {
                    if let Some(Token::Digit) = &prev_token {
                        let val = digits.parse::<u64>().expect("Invalid digits");
                        sum += val;
                        digits.clear();
                        if ch == '*' {
                            gear_ratios
                                .entry((idx, char_idx))
                                .and_modify(|r| {
                                    if r.1 == 0 {
                                        r.0 = val;
                                        r.1 = 1;
                                    } else if r.1 == 1 {
                                        r.0 *= val;
                                        r.1 = 2;
                                    } else {
                                        return;
                                    }
                                })
                                .or_insert((val, 1));
                        }
                    }
                    if ch == '*' {
                        gear_ratios.entry((idx, char_idx)).or_insert((0, 0));
                    }
                    prev_token = Some(Token::Symbol(ch));
                    adjacent_symbol = None;
                }
                _ => continue,
            }
        }
        if adjacent_symbol.is_some()
            && prev_token
                .as_ref()
                .is_some_and(|t| matches!(t, Token::Digit))
        {
            let val = digits.parse::<u64>().expect("Invalid digits");
            sum += val;
            digits.clear();

            let symbol = adjacent_symbol.unwrap();
            if symbol.value == '*' {
                gear_ratios
                    .entry((symbol.row, symbol.col))
                    .and_modify(|r| {
                        if r.1 == 0 {
                            r.0 = val;
                            r.1 = 1;
                        } else if r.1 == 1 {
                            r.0 *= val;
                            r.1 = 2;
                        } else {
                            return;
                        }
                    })
                    .or_insert((val, 1));
            }
        }
        prev = Some(chars);
        idx += 1;
    }
    println!("Sum: {}", sum);
    let mut total_ratios = 0;
    for ratio in &gear_ratios {
        if ratio.1 .1 == 2 {
            total_ratios += ratio.1 .0;
        }
    }
    println!("Gear ratios sum: {}", total_ratios);
}

fn is_symbol(ch: &char) -> bool {
    matches!(ch, '!'..='/' | ':'..='@') && !matches!(ch, '.')
}

fn extract_symbol(chars: &Vec<char>, idx: usize) -> Option<(char, usize)> {
    let min = if idx > 0 { idx - 1 } else { idx };
    let max = if idx + 1 < chars.len() { idx + 1 } else { idx };
    for i in min..=max {
        if is_symbol(&chars[i]) {
            return Some((chars[i], i));
        }
    }
    None
}

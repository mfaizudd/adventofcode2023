use std::{
    collections::{BTreeMap, HashSet},
    fs,
};

fn main() {
    let content = fs::read_to_string("input.txt").expect("Invalid input");
    let mut cards = BTreeMap::new();
    let mut sum = 0;
    let starting_cards = content.lines().count() as i32;
    for line in content.lines() {
        let colon_index = line.find(':').expect("No colon found");
        let card_no = line[4..colon_index]
            .trim()
            .parse::<i32>()
            .expect("Failed to parse card number");
        let total_copies = cards.entry(card_no).or_insert(1);
        let mut chars = line.chars();
        chars.nth(colon_index);
        let mut digits = String::new();
        let mut winning = HashSet::new();
        let mut parsing_winning = true;
        let mut points = 0;
        let mut total_win = 0;
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
                        total_win += 1;
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
                total_win += 1;
            }
        }
        // println!("Card {card_no} total win: {total_win}");
        sum += points;
        let max = std::cmp::min(card_no+total_win, starting_cards);
        if card_no + 1 > starting_cards {
            continue;
        }
        for _ in 0..*total_copies {
            for i in (card_no + 1)..=max {
                cards.entry(i).or_insert(1);
                cards.entry(i).and_modify(|c| *c += 1);
            }
        }
    }
    println!("Sum: {sum}");
    let mut total_cards = 0;
    for (_, v) in cards {
        // println!("Card {k}: {v}");
        total_cards += v;
    }
    println!("Total scratchcards: {total_cards}");
}

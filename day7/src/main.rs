use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut ranks = BTreeMap::new();
    for line in content.lines() {
        let pair = line.split(' ').collect::<Vec<_>>();
        let card = read_card(pair[0]);
        let bid = pair[1].parse::<u64>().unwrap();
        ranks.insert(card, bid);
    }
    let mut result = 0;
    for (i, (_, v)) in ranks.iter().enumerate() {
        // println!("{} * {v} ({k})", i + 1);
        result += (i as u64 + 1) * *v;
    }
    println!("{result}");
}

fn read_card(input: &str) -> u64 {
    let mapping = HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10)]);
    let mut result: u64 = 0;
    let mut map = HashMap::new();
    for ch in input.chars() {
        match ch {
            '0'..='9' | 'A' | 'K' | 'Q' | 'J' | 'T' => {
                map.entry(ch).and_modify(|c| *c += 1).or_insert(1);
            }
            _ => panic!("What the hell is this?"),
        }
    }
    let values = map.values();
    if map.len() == 1 {
        result = 7;
    } else if map.len() == 2 && values.clone().any(|v| *v == 4) && values.clone().any(|v| *v == 1) {
        result = 6;
    } else if map.len() == 2 && values.clone().any(|v| *v == 3) && values.clone().any(|v| *v == 2) {
        result = 5;
    } else if map.len() == 3 && values.clone().any(|v| *v == 3) {
        result = 4;
    } else if map.len() == 3 && values.clone().filter(|v| **v == 2).count() == 2 {
        result = 3;
    } else if map.len() == 4 && values.clone().filter(|v| **v == 2).count() == 1 {
        result = 2;
    } else if map.len() == 5 {
        result = 1;
    }
    // println!(
    //     "{input}: {map:?} {result}, ({}) :: values({values:?})",
    //     values.clone().filter(|v| **v == 1).count()
    // );
    for ch in input.chars() {
        match ch {
            '0'..='9' => result = result * 100 + ch.to_digit(10).unwrap() as u64,
            'A' | 'K' | 'Q' | 'J' | 'T' => result = result * 100 + mapping[&ch],
            _ => panic!("What the hell is this?"),
        }
    }
    result
}

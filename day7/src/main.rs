use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut ranks = BTreeMap::new();
    let mut ranks_j = BTreeMap::new();
    for line in content.lines() {
        let pair = line.split(' ').collect::<Vec<_>>();
        let card = read_card(pair[0]);
        let bid = pair[1].parse::<u64>().unwrap();
        ranks.insert(card, bid);
        ranks_j.insert(read_card_j_mode(pair[0]), bid);
    }
    let mut result = 0;
    for (i, (_, v)) in ranks.iter().enumerate() {
        // println!("{} * {v} ({k})", i + 1);
        result += (i as u64 + 1) * *v;
    }
    let mut result_j = 0;
    for (i, (_, v)) in ranks_j.iter().enumerate() {
        result_j += (i as u64 + 1) * *v;
    }
    println!("{result}");
    println!("{result_j}");
}

fn read_card(input: &str) -> u64 {
    let mapping = HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10)]);
    let mut map = HashMap::new();
    for ch in input.chars() {
        match ch {
            '0'..='9' | 'A' | 'K' | 'Q' | 'J' | 'T' => {
                map.entry(ch).and_modify(|c| *c += 1).or_insert(1);
            }
            _ => panic!("What the hell is this?"),
        }
    }
    let values = map.values().map(|v| *v).collect();
    let mut result = extract_type(map.len(), 0, &values);
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

fn read_card_j_mode(input: &str) -> u64 {
    let mapping = HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', 1), ('T', 10)]);
    let mut map = HashMap::new();
    for ch in input.chars() {
        match ch {
            '0'..='9' | 'A' | 'K' | 'Q' | 'J' | 'T' => {
                map.entry(ch).and_modify(|c| *c += 1).or_insert(1);
            }
            _ => panic!("What the hell is this?"),
        }
    }
    let values = map.values().map(|v| *v).collect();
    let j_count = *map.get(&'J').unwrap_or(&0);
    let mut result = extract_type(map.len(), j_count, &values);
    // println!(
    //     "{input}: {map:?} {result}, ({}) :: values({values:?})",
    //     values.iter().filter(|v| **v == 1).count()
    // );
    for ch in input.chars() {
        match ch {
            '0'..='9' => result = result * 100 + ch.to_digit(10).unwrap() as u64,
            'A' | 'K' | 'Q' | 'J' | 'T' => result = result * 100 + mapping[&ch],
            _ => panic!("What the hell is this?"),
        }
    }
    // println!("{input}: {result}");
    result
}

fn extract_type(count: usize, j_count: i32, values: &Vec<i32>) -> u64 {
    // five of a kind
    if count == 1 {
        7
    // four of a kind
    } else if count == 2 && values.iter().any(|v| *v == 4) && values.iter().any(|v| *v == 1) {
        match j_count {
            1 | 4 => return 7,
            _ => return 6,
        }
    // full house
    } else if count == 2 && values.iter().any(|v| *v == 3) && values.iter().any(|v| *v == 2) {
        match j_count {
            2 | 3 => return 7,
            _ => return 5,
        }
    // three of a kind
    } else if count == 3 && values.iter().any(|v| *v == 3) {
        match j_count {
            1 | 3 => return 6,
            _ => return 4,
        }
    // two pairs
    } else if count == 3 && values.iter().filter(|v| **v == 2).count() == 2 {
        match j_count {
            1 => return 5,
            2 => return 6,
            _ => return 3,
        }
    // one pair
    } else if count == 4 && values.iter().filter(|v| **v == 2).count() == 1 {
        match j_count {
            1 | 2 => return 4,
            _ => return 2,
        }
    // high card
    } else if count == 5 {
        match j_count {
            1 => return 2,
            _ => return 1,
        }
    // not valid
    } else {
        0
    }
}

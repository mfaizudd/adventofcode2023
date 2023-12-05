use std::{fs, fmt::Debug};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct TaggedInt<'a> {
    value: i64,
    tag: &'a str,
}

impl<'a> Debug for TaggedInt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.value))
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut lines = content.lines();
    let seeds_line = lines.next().unwrap();
    let mut seeds = seeds_line[seeds_line.find(':').unwrap() + 2..]
        .split(' ')
        .map(|s| TaggedInt {
            value: s.parse::<i64>().unwrap(),
            tag: "seed",
        })
        .collect::<Vec<_>>();
    // println!("{seeds:?}");
    let mut tag = "seed";
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.chars().nth(0).is_some_and(|ch| ch.is_alphabetic()) {
            tag = line.split('-').last().unwrap();
            continue;
        }
        let converter = line
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let dest = converter[0];
        let source = converter[1];
        let count = converter[2];
        let offset = dest - source;
        // print!("{line}: \t");
        for seed in &mut seeds {
            if seed.tag != tag && seed.value >= source && seed.value < source + count {
                seed.value += offset;
                seed.tag = tag;
            }
        }
        // println!("{seeds:?}");
    }

    let min = seeds.iter().min().unwrap();
    println!("Min location: {}", min.value);
}

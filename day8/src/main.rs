use std::{collections::BTreeMap, fs};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut lines = content.lines();
    let paths = lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| if ch == 'L' { 0 } else { 1 })
        .collect::<Vec<_>>();
    let mut maps = BTreeMap::new();
    lines.next();
    for line in lines {
        let (current, branch) = extract_path(line);
        maps.insert(current, branch);
    }
    let mut steps = 0;
    let mut pos = "AAA";
    while pos != "ZZZ" {
        let next = paths[steps % paths.len()];
        pos = maps[pos][next];
        println!("{pos}");
        steps += 1;
    }
    println!("{steps}");
}

fn extract_path(input: &str) -> (&str, Vec<&str>) {
    let inputs = input.split(' ').collect::<Vec<_>>();
    let current = inputs[0];
    let left = &inputs[2][1..4];
    let right = &inputs[3][0..3];
    let branch = vec![left, right];
    (current, branch)
}

use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

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
    let mut steps: u64 = 0;
    // Part 1
    // let mut pos = "AAA";
    // while pos != "ZZZ" {
    //     let next = paths[steps % paths.len()];
    //     pos = maps[pos][next];
    //     steps += 1;
    // }
    // println!("{steps}");
    let mut pos_ghost = maps
        .iter()
        .map(|(&c, _)| c)
        .filter(|&c| &c[2..] == "A")
        .collect::<Vec<_>>();
    let mut pos_steps = HashMap::new();
    while pos_steps.len() != pos_ghost.len() {
        let next = paths[steps as usize % paths.len()];
        let mut i = 0;
        for pos in &mut pos_ghost {
            *pos = maps[pos][next];
            if &pos[2..] == "Z" {
                pos_steps.entry(i).or_insert(steps + 1);
            }
            i += 1;
        }
        steps += 1;
    }
    println!("{pos_steps:?}");
    steps = 0;
    for (_, step) in pos_steps {
        steps = if steps == 0 { step } else { lcm(steps, step) };
    }
    println!("ghost_step {steps}");
}

fn extract_path(input: &str) -> (&str, Vec<&str>) {
    let inputs = input.split(' ').collect::<Vec<_>>();
    let current = inputs[0];
    let left = &inputs[2][1..4];
    let right = &inputs[3][0..3];
    let branch = vec![left, right];
    (current, branch)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a,b)) * b
}

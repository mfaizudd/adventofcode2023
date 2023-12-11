use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();
    let lines: Vec<_> = content.lines().collect();
    let mut galaxies = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        // println!("{line}");
        let mut empty = true;
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    galaxies.push((j as i64, i as i64));
                    empty = false;
                }
                _ => continue,
            }
        }
        if empty {
            empty_rows.insert(i as i64);
        }
    }
    let line = *lines.first().unwrap();
    for i in 0..line.len() {
        if !lines.iter().any(|&l| l.chars().nth(i).unwrap() == '#') {
            empty_cols.insert(i as i64);
        }
    }
    let mut sum_distance = 0;
    for source in 0..galaxies.len() {
        for target in source + 1..galaxies.len() {
            // print!("{source} to {target}");
            let source = galaxies[source];
            let target = galaxies[target];
            let distance = calculate_distance(source, target, &empty_rows, &empty_cols);
            // println!(" is {distance}");
            sum_distance += distance;
        }
    }
    println!("{sum_distance}");
}

fn calculate_distance(
    source: (i64, i64),
    target: (i64, i64),
    empty_rows: &HashSet<i64>,
    empty_cols: &HashSet<i64>,
) -> i64 {
    let mut steps = 0;
    let mut current = source;
    let log = source.0 == 3 && source.1 == 0 && target.0 == 7 && target.1 == 8;
    let mut left = true;
    // println!("{source:?} to {target:?}");
    while current != target {
        let diff = get_diff(current, target);
        let diff = if diff.0 != 0 && diff.1 != 0 && left {
            left = false;
            (clamp(target.0 - source.0), 0)
        } else if diff.0 != 0 && diff.1 != 0 && !left {
            left = true;
            (0, clamp(target.1 - source.1))
        } else {
            diff
        };
        current = add(current, diff);
        // println!("{current:?}");
        steps += if diff.0 != 0 && empty_cols.contains(&current.0) {
            2
        } else if diff.1 != 0 && empty_rows.contains(&current.1) {
            2
        } else {
            1
        };
        if log {
            println!("1 to 7 step {steps}: {current:?}");
        }
    }
    steps
}
fn get_diff(source: (i64, i64), target: (i64, i64)) -> (i64, i64) {
    (clamp(target.0 - source.0), clamp(target.1 - source.1))
}

fn add(source: (i64, i64), rhs: (i64, i64)) -> (i64, i64) {
    (source.0 + rhs.0, source.1 + rhs.1)
}

fn clamp(value: i64) -> i64 {
    if value > 0 {
        1
    } else if value < 0 {
        -1
    } else {
        0
    }
}

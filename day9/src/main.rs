use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut prev = 0;
    let mut next = 0;
    for line in content.lines() {
        let (first, last) = extract_prediction(line);
        next += last;
        prev += first;
    }
    println!("{prev} - {next}");
}

fn extract_prediction(input: &str) -> (i64, i64) {
    // print!("{input}: ");
    let mut values = input
        .split(' ')
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut diffs = Vec::new();
    diffs.push(values.clone());
    while !values.iter().all(|&v| v == 0) {
        let mut vals = values.iter();
        let mut prev = vals.next();
        let mut diff = Vec::new();
        while let Some(curr) = vals.next() {
            if let Some(prev) = prev {
                diff.push(curr - prev);
            }
            prev = Some(curr);
        }
        diffs.push(diff.clone());
        values = diff;
    }
    let mut prev_diff_asc = None;
    let mut prev_diff_desc = None;
    for diff in diffs.iter_mut().rev() {
        let last = *diff.last().unwrap();
        let first = *diff.first().unwrap();
        if let (Some(prev_desc), Some(prev_asc)) = (prev_diff_desc, prev_diff_asc) {
            prev_diff_desc = Some(first - prev_desc);
            prev_diff_asc = Some(last + prev_asc);
            diff.insert(0, first - prev_desc);
            diff.push(last + prev_asc);
            // print!("{diff:?},");
            continue;
        }
        prev_diff_asc = Some(last);
        prev_diff_desc = Some(first);
    }
    // println!("");
    let original = diffs.first().unwrap();
    // println!("{original:?}");
    let first = *original.first().unwrap();
    let last = *original.last().unwrap();
    (first, last)
}

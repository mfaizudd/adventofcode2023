use std::{fmt::Debug, fs};

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

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct SeedRange<'a> {
    start: i64,
    end: i64,
    tag: &'a str,
}

impl<'a> Debug for SeedRange<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Seed({} - {})", self.start, self.end))
    }
}

struct Converter<'a> {
    start: i64,
    end: i64,
    offset: i64,
    tag: &'a str,
}

impl<'a> Debug for Converter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Converter[{}]({} - {})",
            self.tag, self.start, self.end
        ))
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut lines = content.lines();
    let seeds_line = lines.next().unwrap();
    let mut seeds_pairs = seeds_line[seeds_line.find(':').unwrap() + 2..]
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap());
    let mut seeds = Vec::new();
    while let Some(seed) = seeds_pairs.next() {
        let count = seeds_pairs.next().unwrap();
        seeds.push(SeedRange {
            start: seed,
            end: seed + count - 1,
            tag: "seed",
        });
    }
    // println!("{seeds:?}");
    let mut converters = Vec::new();
    let mut tag = "seed";
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.chars().nth(0).is_some_and(|ch| ch.is_alphabetic()) {
            tag = line.split(' ').next().unwrap().split('-').last().unwrap();
            if converters.is_empty() {
                continue;
            }
            // println!("{converters:?}");
            let mut new_seeds = Vec::new();
            for seed in &seeds {
                new_seeds.extend(convert(seed, &converters));
            }
            // println!("{new_seeds:?}");
            seeds = new_seeds;
            converters.clear();
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
        converters.push(Converter {
            start: source,
            end: source + count,
            offset,
            tag,
        });

        // print!("{line}: \t");
        // for seed in &mut seeds {
        //     if seed.tag != tag && seed.value >= source && seed.value < source + count {
        //         seed.value += offset;
        //         seed.tag = tag;
        //     }
        // }
        // println!("{seeds:?}");
    }
    if !converters.is_empty() {
        // println!("{converters:?}");
        let mut new_seeds = Vec::new();
        for seed in &seeds {
            new_seeds.extend(convert(seed, &converters));
        }
        // println!("{new_seeds:?}");
        seeds = new_seeds;
    }

    let min = seeds.iter().min().unwrap();
    println!("Min location: {}", min.start);
}

fn convert<'a>(seed: &SeedRange<'a>, converters: &Vec<Converter<'a>>) -> Vec<SeedRange<'a>> {
    let mut result = Vec::new();
    for converter in converters {
        if seed.tag != converter.tag && seed.start >= converter.start && seed.start < converter.end
        {
            if seed.end < converter.end {
                result.push(SeedRange {
                    start: seed.start + converter.offset,
                    end: seed.end + converter.offset,
                    tag: converter.tag,
                });
            } else {
                let seed = SeedRange {
                    start: converter.end,
                    end: seed.end,
                    tag: seed.tag,
                };
                result.extend(convert(&seed, converters));
            }
        } else if seed.tag != converter.tag
            && seed.end >= converter.start
            && seed.start < converter.end
        {
            result.push(SeedRange {
                start: converter.start + converter.offset,
                end: seed.end + converter.offset,
                tag: converter.tag,
            });
            let seed = SeedRange {
                start: seed.start,
                end: converter.start - 1,
                tag: seed.tag,
            };
            result.extend(convert(&seed, converters));
        }
    }
    if result.is_empty() {
        result.push(SeedRange {
            start: seed.start,
            end: seed.end,
            tag: converters[0].tag,
        })
    }
    result
}

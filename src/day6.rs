use std::collections::HashMap;
use std::env;
use std::process;

use common::load_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day6 <file>");
        process::exit(1);
    }
    let lines: Vec<String> = load_file(&args[1]);

    let initial_state: Vec<u8> = lines
        .first()
        .unwrap()
        .split(",")
        .into_iter()
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let mut per_day: HashMap<u8, usize> = HashMap::new();

    for day in initial_state {
        *per_day.entry(day).or_insert(0) += 1;
    }

    println!("Part A: {:}", simulate(per_day.clone(), 80));
    println!("Part B: {:}", simulate(per_day.clone(), 256));
}

fn simulate(per_day: HashMap<u8, usize>, days: usize) -> usize {
    let mut next = per_day;

    for _ in 0..days {
        next = next_day(next);
    }

    next.values().into_iter().sum()
}

fn next_day(per_day: HashMap<u8, usize>) -> HashMap<u8, usize> {
    let next: Vec<(u8, usize)> = per_day
        .iter()
        .map(|(&cycle_days, &fish_count)| match cycle_days {
            0 => vec![(6, fish_count), (8, fish_count)],
            n => vec![(n - 1, fish_count)],
        })
        .flatten()
        .collect();

    let mut map: HashMap<u8, usize> = HashMap::new();

    for (day, count) in next {
        *map.entry(day).or_insert(0) += count;
    }

    map
}

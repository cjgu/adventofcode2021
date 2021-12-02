use std::env;
use std::process;

use common::load_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day1 <file>");
        process::exit(1);
    }
    let depths: Vec<i32> = load_file(&args[1])
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    part1(&depths);
    part2(&depths);
    part2_clever(&depths);
}

fn part1(depths: &Vec<i32>) {
    let increases = depths
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count();
    println!("Part 1: {:?}", increases);
}

fn part2(depths: &Vec<i32>) {
    let mut increases = 0;

    let moving_sums: Vec<i32> = depths.windows(3).map(|s| s.into_iter().sum()).collect();
    for s in moving_sums.windows(2) {
        if s[1] > s[0] {
            increases += 1;
        }
    }

    println!("Part 2: {:?}", increases);
}

fn part2_clever(depths: &Vec<i32>) {
    let increases = depths
        .windows(4)
        .filter(|window| window[3] > window[0])
        .count();

    println!("Part 2: {:?}", increases);
}

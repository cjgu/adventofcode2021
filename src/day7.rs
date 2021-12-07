use std::env;
use std::process;

use common::load_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day7 <file>");
        process::exit(1);
    }
    let mut numbers: Vec<u32> = load_file(&args[1])
        .first()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    numbers.sort();

    part_a(&numbers);
    part_b(&numbers);
}

fn part_a(numbers: &Vec<u32>) {
    let midpoint = numbers.len() / 2;

    let median = numbers[midpoint];

    let sum: i64 = numbers
        .into_iter()
        .map(|&x| (median as i64 - x as i64).abs())
        .sum();

    println!("Part A: {:}", sum);
}

fn part_b(numbers: &Vec<u32>) {
    let sum: u32 = numbers.into_iter().sum();
    let mean: u32 = sum / numbers.into_iter().count() as u32;

    let sum: u64 = numbers
        .into_iter()
        .map(|&x| cost_b((mean as i64 - x as i64).abs() as u64))
        .sum();

    println!("Part A: {:}", sum);
}

fn cost_b(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

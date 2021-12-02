use std::env;
use std::process;

use common::load_file;

enum Command {
    Up(i64),
    Down(i64),
    Forward(i64),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day2 <file>");
        process::exit(1);
    }

    let commands: Vec<Command> = load_file(&args[1])
        .into_iter()
        .map(|command| {
            let parts: Vec<&str> = command.split(' ').collect();
            let value = parts[1].parse::<i64>().unwrap();

            match parts[0] {
                "forward" => Command::Forward(value),
                "up" => Command::Up(value),
                "down" => Command::Down(value),
                _ => panic!("unknown command"),
            }
        })
        .collect();

    part1(&commands);
    part2(&commands);
}

fn part1(commands: &Vec<Command>) {
    let mut depth = 0;
    let mut x = 0;

    for command in commands {
        match &command {
            Command::Up(value) => depth -= value,
            Command::Down(value) => depth += value,
            Command::Forward(value) => x += value,
        }
    }

    println!(
        "Part1 Depth: {:}, Position: {:}, multiplied: {:}",
        depth,
        x,
        depth * x
    );
}

fn part2(commands: &Vec<Command>) {
    let mut depth = 0;
    let mut aim = 0;
    let mut position = 0;

    for command in commands {
        match &command {
            Command::Up(value) => aim -= value,
            Command::Down(value) => aim += value,
            Command::Forward(value) => {
                position += value;
                depth += aim * value;
            }
        }
    }

    println!(
        "Part2 Depth: {:}, Position: {:}, Aim: {:}, multiplied: {:}",
        depth,
        position,
        aim,
        depth * position
    );
}

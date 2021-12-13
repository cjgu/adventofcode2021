use std::env;
use std::process;

use common::load_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day10 <file>");
        process::exit(1);
    }
    let input: Vec<String> = load_file(&args[1]);

    part_a(&input);
    part_b(&input);
}

fn part_a(lines: &Vec<String>) {
    let mut corrupted_sum: u64 = 0;
    for line in lines {
        match parse_line(line) {
            Status::Corrupted(x) => {
                corrupted_sum += x;
            }
            Status::Incomplete(_) => {}
            Status::Complete => {}
        }
    }
    println!("Part A: {:}", corrupted_sum);
}

fn part_b(lines: &Vec<String>) {
    let mut scores: Vec<u64> = Vec::new();

    for line in lines {
        match parse_line(line) {
            Status::Corrupted(_) => {}
            Status::Incomplete(x) => {
                scores.push(score_missing(&x));
            }
            Status::Complete => {}
        }
    }

    scores.sort();

    println!("Part B: {:}", scores[scores.len() / 2]);
}

#[derive(PartialEq, Eq, Debug)]
enum Status {
    Corrupted(u64),
    Incomplete(Vec<char>),
    Complete,
}

fn parse_line(line: &String) -> Status {
    let mut stack = Vec::<char>::new();

    for c in line.chars() {
        match c {
            '(' => stack.push(c),
            ')' => match stack.pop() {
                Some('(') => {
                    continue;
                }
                Some(x) => {
                    return Status::Corrupted(points(')'));
                }
                None => {
                    let mut missing = stack.clone();
                    missing.reverse();
                    return Status::Incomplete(missing);
                }
            },
            '{' => stack.push(c),
            '}' => match stack.pop() {
                Some('{') => {
                    continue;
                }
                Some(x) => return Status::Corrupted(points('}')),
                None => {
                    let mut missing = stack.clone();
                    missing.reverse();
                    return Status::Incomplete(missing);
                }
            },
            '[' => stack.push(c),
            ']' => match stack.pop() {
                Some('[') => {
                    continue;
                }
                Some(x) => return Status::Corrupted(points(']')),
                None => {
                    let mut missing = stack.clone();
                    missing.reverse();
                    return Status::Incomplete(missing);
                }
            },
            '<' => stack.push(c),
            '>' => match stack.pop() {
                Some('<') => {
                    continue;
                }
                Some(x) => return Status::Corrupted(points('>')),
                None => {
                    let mut missing = stack.clone();
                    missing.reverse();
                    return Status::Incomplete(missing);
                }
            },
            _ => {
                continue;
            }
        }
    }

    if stack.len() == 0 {
        Status::Complete
    } else {
        let mut missing = stack.clone();
        missing.reverse();
        Status::Incomplete(missing)
    }
}

fn points(c: char) -> u64 {
    match c {
        ')' => 3,
        '}' => 1197,
        ']' => 57,
        '>' => 25137,
        _ => 0,
    }
}

fn score_missing(missing: &Vec<char>) -> u64 {
    let mut score = 0;

    for c in missing {
        score *= 5;

        score += match c {
            '(' => 1,
            '{' => 3,
            '[' => 2,
            '<' => 4,
            _ => 0,
        };
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(&"()".to_owned()), Status::Complete);
        assert_eq!(parse_line(&"(".to_owned()), Status::Incomplete(vec!['(']));
        assert_eq!(
            parse_line(&"((".to_owned()),
            Status::Incomplete(vec!['(', '('])
        );
        assert_eq!(
            parse_line(&"((<".to_owned()),
            Status::Incomplete(vec!['<', '(', '('])
        );
        assert_eq!(parse_line(&"(>".to_owned()), Status::Corrupted(points('>')));
    }

    #[test]
    fn test_score() {
        assert_eq!(score_missing(&vec![]), 0);
        assert_eq!(score_missing(&vec!['[', '(', '{', '<']), 294);
    }
}

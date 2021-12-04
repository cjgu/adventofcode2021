use std::collections::HashSet;
use std::env;
use std::process;

use common::load_file;

struct Board {
    lines: Vec<Vec<u64>>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day4 <file>");
        process::exit(1);
    }

    let lines: Vec<String> = load_file(&args[1]);

    let numbers = lines
        .first()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let board_lines = &lines[1..];

    let boards = board_lines
        .chunks(6)
        .map(|lines| Board {
            lines: lines[1..].into_iter().map(|row| parse_row(row)).collect(),
        })
        .collect();

    part_a(&numbers, &boards);
    part_b(&numbers, &boards);
}

fn part_a(numbers: &Vec<u64>, boards: &Vec<Board>) {
    let (found_board_id, last_number_index) = find_first_board_score(&numbers, &boards).unwrap();

    let drawn_numbers: Vec<&u64> = numbers[0..last_number_index].into_iter().collect();
    let last_drawn = numbers[last_number_index - 1];

    let score = calculate_score(&boards[found_board_id], drawn_numbers);

    println!("Part A: {:}", score * last_drawn);
}

fn part_b(numbers: &Vec<u64>, boards: &Vec<Board>) {
    let (found_board_id, last_number_index) = find_last_board_score(&numbers, &boards);
    let drawn_numbers: Vec<&u64> = numbers[0..last_number_index].into_iter().collect();
    let last_drawn = numbers[last_number_index - 1];

    let score = calculate_score(&boards[found_board_id], drawn_numbers);
    println!("Part B: {:}", score * last_drawn);
}

fn find_first_board_score(numbers: &Vec<u64>, boards: &Vec<Board>) -> Option<(usize, usize)> {
    for i in 1..numbers.len() {
        let drawn: HashSet<&u64> = numbers[0..i].iter().collect();
        for (board_id, board) in boards.into_iter().enumerate() {
            if score_board(&drawn, board) {
                return Some((board_id, i));
            }
        }
    }

    None
}

fn find_last_board_score(numbers: &Vec<u64>, boards: &Vec<Board>) -> (usize, usize) {
    let mut win_order: Vec<usize> = Vec::new();
    let mut won: HashSet<usize> = HashSet::new();
    let mut last_number_index = 0;

    for i in 1..numbers.len() {
        let drawn: HashSet<&u64> = numbers[0..i].iter().collect();
        for (board_id, board) in boards.into_iter().enumerate() {
            if won.contains(&board_id) {
                continue;
            }
            if score_board(&drawn, board) {
                win_order.push(board_id);
                won.insert(board_id);
            }
        }
        if won.len() == boards.len() {
            last_number_index = i;
            break;
        }
    }

    (*win_order.last().unwrap(), last_number_index)
}

fn calculate_score(board: &Board, drawn_numbers: Vec<&u64>) -> u64 {
    let drawn: HashSet<&u64> = drawn_numbers.into_iter().collect();

    let mut sum = 0;
    for line in &board.lines {
        for col in line {
            if !drawn.contains(col) {
                sum += col;
            }
        }
    }
    return sum;
}

fn score_board(numbers: &HashSet<&u64>, board: &Board) -> bool {
    for line in 0..board.lines.len() {
        if score_line(numbers, board, line) {
            return true;
        }
    }

    for column in 0..board.lines[0].len() {
        if score_column(numbers, board, column) {
            return true;
        }
    }
    return false;
}

fn score_line(numbers: &HashSet<&u64>, board: &Board, line: usize) -> bool {
    let line = &board.lines[line];

    line.into_iter()
        .filter(|&number| numbers.contains(&number))
        .collect::<Vec<&u64>>()
        .len()
        == 5
}
fn score_column(numbers: &HashSet<&u64>, board: &Board, column: usize) -> bool {
    board
        .lines
        .clone()
        .into_iter()
        .filter(|line| numbers.contains(&line[column]))
        .collect::<Vec<Vec<u64>>>()
        .len()
        == 5
}

fn parse_row(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        assert_eq!(parse_row("1"), vec![1]);
        assert_eq!(parse_row(" 1 20 99  2"), vec![1, 20, 99, 2]);
    }
}

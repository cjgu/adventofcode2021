use std::collections::HashSet;
use std::env;
use std::process;

use common::load_file;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day10 <file>");
        process::exit(1);
    }
    let input: Vec<String> = load_file(&args[1]);

    let split_pos = input.iter().position(|l| l == "").unwrap();

    let positions = input[0..split_pos].to_vec();
    let folds = input[split_pos + 1..].to_vec();

    let positions = parse_positions(&positions);
    let folds = parse_folds(&folds);

    part_a(&positions, &folds);
    part_b(&positions, &folds);
}

fn part_a(positions: &HashSet<Position>, folds: &Vec<Fold>) {
    let after = fold(&positions, folds[0]);
    println!("Part A: {:?}", after.len());
}

fn part_b(positions: &HashSet<Position>, folds: &Vec<Fold>) {
    let mut after = positions.clone();

    let mut last_x_fold = 0;
    let mut last_y_fold = 0;

    for &f in folds {
        after = fold(&after, f);
        match f {
            Fold::X(x) => {
                last_x_fold = x;
            }
            Fold::Y(y) => {
                last_y_fold = y;
            }
        }
    }

    println!("Part B");
    for y in 0..last_y_fold {
        for x in 0..last_x_fold {
            if after.contains(&Position { x: x, y: y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

fn fold(positions: &HashSet<Position>, fold: Fold) -> HashSet<Position> {
    let mut new_positions: HashSet<Position> = HashSet::new();

    for position in positions {
        if let Some(pos) = fold_position(position, fold) {
            new_positions.insert(pos);
        }
    }

    new_positions
}

fn fold_position(pos: &Position, fold: Fold) -> Option<Position> {
    match fold {
        Fold::X(x) => {
            if pos.x < x {
                Some(*pos)
            } else if pos.x == x {
                // do nothing
                None
            } else {
                Some(Position {
                    x: x - 1 - (pos.x - x - 1),
                    y: pos.y,
                })
            }
        }
        Fold::Y(y) => {
            if pos.y < y {
                Some(*pos)
            } else if pos.y == y {
                // do nothing
                None
            } else {
                Some(Position {
                    x: pos.x,
                    y: y - 1 - (pos.y - y - 1),
                })
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn parse_positions(positions: &Vec<String>) -> HashSet<Position> {
    let mut p = HashSet::new();

    for pos in positions {
        let pos: Vec<&str> = pos.split(",").collect();

        p.insert(Position {
            x: pos[0].parse::<usize>().unwrap(),
            y: pos[1].parse::<usize>().unwrap(),
        });
    }

    p
}

#[derive(Copy, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

fn parse_folds(folds: &Vec<String>) -> Vec<Fold> {
    let mut f = Vec::new();

    let re = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
    for fold in folds {
        let captures = re.captures(fold).unwrap();

        let axis = captures.get(1).unwrap().as_str();
        let pos = captures.get(2).unwrap().as_str();

        match axis {
            "x" => f.push(Fold::X(pos.parse::<usize>().unwrap())),
            "y" => f.push(Fold::Y(pos.parse::<usize>().unwrap())),
            _ => {}
        }
    }

    f
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fold_position_x() {
        assert_eq!(
            fold_position(&Position { x: 4, y: 0 }, Fold::X(2)),
            Some(Position { x: 0, y: 0 })
        );
        assert_eq!(
            fold_position(&Position { x: 3, y: 0 }, Fold::X(2)),
            Some(Position { x: 1, y: 0 })
        );
    }

    #[test]
    fn test_fold_position_y() {
        assert_eq!(
            fold_position(&Position { x: 0, y: 4 }, Fold::Y(2)),
            Some(Position { x: 0, y: 0 })
        );
        assert_eq!(
            fold_position(&Position { x: 0, y: 3 }, Fold::Y(2)),
            Some(Position { x: 0, y: 1 })
        );
    }
}

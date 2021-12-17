use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::process;

use common::load_file;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Velocity {
    dx: i64,
    dy: i64,
}

impl Velocity {
    fn decrease(&self) -> Velocity {
        Velocity {
            dx: cmp::max(0, self.dx - 1),
            dy: self.dy - 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn add(&self, vec: &Velocity) -> Coord {
        Coord {
            x: self.x + vec.dx,
            y: self.y + vec.dy,
        }
    }
}

#[derive(Debug)]
struct Area {
    top_left: Coord,
    bottom_right: Coord,
}

impl Area {
    fn is_inside_area(&self, coord: &Coord) -> bool {
        coord.x >= self.top_left.x
            && coord.x <= self.bottom_right.x
            && coord.y <= self.top_left.y
            && coord.y >= self.bottom_right.y
    }

    fn has_overshot(&self, coord: &Coord) -> bool {
        coord.x > self.bottom_right.x || coord.y < self.bottom_right.y
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day17 <file>");
        process::exit(1);
    }
    let lines: Vec<String> = load_file(&args[1]);

    let target_area = parse_area(&lines[0]);
    let start = Coord { x: 0, y: 0 };

    search_max(start, target_area);
}

fn parse_area(str: &str) -> Area {
    let re = Regex::new(r"^target area: x=([-]*\d+)..([-]*\d+), y=([-]*\d+)..([-]*\d+)$").unwrap();

    let captures = re.captures(str).unwrap();

    let x1 = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let x2 = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
    let y1 = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
    let y2 = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();

    let top_left = Coord {
        x: cmp::min(x1, x2),
        y: cmp::max(y1, y2),
    };
    let bottom_right = Coord {
        x: cmp::max(x1, x2),
        y: cmp::min(y1, y2),
    };

    Area {
        top_left,
        bottom_right,
    }
}

enum Score {
    Hit(i64),
    Miss,
}

fn search_max(start: Coord, target: Area) {
    let mut max_y = 0;
    let mut hits = 0;

    for dx in 1..1000 {
        for dy in -200..1000 {
            match trace(&start, Velocity { dx, dy }, &target) {
                Score::Hit(my) => {
                    if my > max_y {
                        max_y = my;
                    }
                    hits += 1;
                }
                Score::Miss => {}
            }
        }
    }

    println!("Part A: {:?}", max_y);
    println!("Part B: {:?}", hits);
}

fn trace(start: &Coord, velocity: Velocity, target: &Area) -> Score {
    let mut cur_pos = start.clone();
    let mut cur_vel = velocity;
    let mut max_y = 0;

    loop {
        let new_pos = cur_pos.add(&cur_vel);
        if target.has_overshot(&cur_pos) {
            return Score::Miss;
        }

        if new_pos.y > max_y {
            max_y = new_pos.y;
        }

        if target.is_inside_area(&new_pos) {
            return Score::Hit(max_y);
        }

        cur_vel = cur_vel.decrease();
        cur_pos = new_pos;
    }
}

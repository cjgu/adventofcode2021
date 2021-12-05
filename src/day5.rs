use std::cmp;
use std::collections::HashMap;
use std::env;
use std::process;

use common::load_file;

#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
struct Coord {
    x: u64,
    y: u64,
}

impl From<&str> for Coord {
    fn from(item: &str) -> Coord {
        let a: Vec<&str> = item.split(",").collect();
        Coord {
            x: a[0].parse::<u64>().unwrap(),
            y: a[1].parse::<u64>().unwrap(),
        }
    }
}

struct Line {
    a: Coord,
    b: Coord,
}

impl Line {
    fn vertical(&self) -> bool {
        self.a.x == self.b.x
    }
    fn horizontal(&self) -> bool {
        self.a.y == self.b.y
    }
    fn diagonal(&self) -> bool {
        self.a.x != self.b.x && self.a.y != self.b.y
    }

    fn produce_covered_coords(&self, include_diagonal: bool) -> Vec<Coord> {
        if self.vertical() {
            let low = cmp::min(self.a.y, self.b.y);
            let high = cmp::max(self.a.y, self.b.y);
            (low..high + 1).map(|y| Coord { x: self.a.x, y }).collect()
        } else if self.horizontal() {
            let low = cmp::min(self.a.x, self.b.x);
            let high = cmp::max(self.a.x, self.b.x);
            (low..high + 1)
                .map(|x| Coord { x: x, y: self.a.y })
                .collect()
        } else if self.diagonal() && include_diagonal {
            let (start_coord, end_coord) = if self.a.x < self.b.x {
                (self.a, self.b)
            } else {
                (self.b, self.a)
            };

            (start_coord.x..end_coord.x + 1)
                .enumerate()
                .map(|(index, x)| Coord {
                    x: x,
                    y: if start_coord.y < end_coord.y {
                        start_coord.y + index as u64
                    } else {
                        start_coord.y - index as u64
                    },
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl From<String> for Line {
    fn from(item: String) -> Line {
        let coords: Vec<&str> = item.split(" -> ").collect();

        Line {
            a: Coord::from(coords[0]),
            b: Coord::from(coords[1]),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day5 <file>");
        process::exit(1);
    }
    let lines: Vec<Line> = load_file(&args[1])
        .into_iter()
        .map(|line| Line::from(line))
        .collect();

    part_a(&lines);
    part_b(&lines);
}

fn part_a(lines: &[Line]) {
    let layout = layout_lines(lines, false);
    let intersecting_lines = count_dangerous_spots(&layout);
    println!("Part A: {:}", intersecting_lines);
}

fn part_b(lines: &[Line]) {
    let layout = layout_lines(lines, true);
    let intersecting_lines = count_dangerous_spots(&layout);
    println!("Part B: {:}", intersecting_lines);
}

fn count_dangerous_spots(layout: &HashMap<Coord, usize>) -> usize {
    layout
        .values()
        .filter(|&intersecting_lines| intersecting_lines >= &2)
        .count()
}

fn layout_lines(lines: &[Line], include_horizontal: bool) -> HashMap<Coord, usize> {
    let mut layout = HashMap::new();

    for line in lines {
        for coord in line.produce_covered_coords(include_horizontal) {
            *layout.entry(coord).or_insert(0) += 1;
        }
    }

    layout
}

fn print_layout(layout: &HashMap<Coord, usize>, size: usize) {
    for y in 0..size + 1 {
        for x in 0..size + 1 {
            let val = layout.get(&Coord {
                x: x as u64,
                y: y as u64,
            });
            match val {
                Some(v) => print!("{:} ", v),
                _ => print!(". "),
            }
        }
        println!("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let lines = vec![
            Line::from("1,1 -> 1,3".to_owned()),
            Line::from("1,1 -> 3,1".to_owned()),
        ];
        let layout = layout_lines(&lines, false);
        assert_eq!(layout.get(&Coord { x: 1, y: 1 }).unwrap(), &2);
        assert_eq!(layout.get(&Coord { x: 1, y: 2 }).unwrap(), &1);
        assert_eq!(layout.get(&Coord { x: 1, y: 3 }).unwrap(), &1);
        assert_eq!(layout.get(&Coord { x: 2, y: 1 }).unwrap(), &1);
        assert_eq!(layout.get(&Coord { x: 3, y: 1 }).unwrap(), &1);
        assert_eq!(layout.len(), 5);

        assert_eq!(count_dangerous_spots(&layout), 1);

        let lines = vec![
            Line::from("1,3 -> 1,1".to_owned()),
            Line::from("1,1 -> 3,1".to_owned()),
        ];
        let layout = layout_lines(&lines, false);
        assert_eq!(layout.get(&Coord { x: 1, y: 1 }).unwrap(), &2);
        assert_eq!(layout.get(&Coord { x: 1, y: 2 }).unwrap(), &1);
        assert_eq!(layout.get(&Coord { x: 1, y: 3 }).unwrap(), &1);
        assert_eq!(layout.get(&Coord { x: 2, y: 1 }).unwrap(), &1);
        assert_eq!(layout.get(&Coord { x: 3, y: 1 }).unwrap(), &1);
        assert_eq!(layout.len(), 5);

        assert_eq!(count_dangerous_spots(&layout), 1);

        let lines = vec![
            Line::from("1,3 -> 1,1".to_owned()),
            Line::from("1,1 -> 3,1".to_owned()),
            Line::from("3,3 -> 3,1".to_owned()),
            Line::from("1,1 -> 1,2".to_owned()),
        ];
        let layout = layout_lines(&lines, false);
        assert_eq!(layout.len(), 7);

        assert_eq!(count_dangerous_spots(&layout), 3);
    }

    #[test]
    fn test_example_a() {
        let lines = vec![
            Line::from("0,9 -> 5,9".to_owned()),
            Line::from("8,0 -> 0,8".to_owned()),
            Line::from("9,4 -> 3,4".to_owned()),
            Line::from("2,2 -> 2,1".to_owned()),
            Line::from("7,0 -> 7,4".to_owned()),
            Line::from("6,4 -> 2,0".to_owned()),
            Line::from("0,9 -> 2,9".to_owned()),
            Line::from("3,4 -> 1,4".to_owned()),
            Line::from("0,0 -> 8,8".to_owned()),
            Line::from("5,5 -> 8,2".to_owned()),
        ];
        let layout = layout_lines(&lines, false);

        assert_eq!(count_dangerous_spots(&layout), 5);
    }

    #[test]
    fn test_line_from() {
        let line = Line::from("893,613 -> 380,613".to_owned());
        assert_eq!(line.a, Coord { x: 893, y: 613 });
        assert_eq!(line.b, Coord { x: 380, y: 613 });
    }

    #[test]
    fn test_produce_line_coords() {
        let horizontal = Line {
            a: Coord { x: 1, y: 1 },
            b: Coord { x: 3, y: 1 },
        };

        assert_eq!(
            horizontal.produce_covered_coords(false),
            vec![
                Coord { x: 1, y: 1 },
                Coord { x: 2, y: 1 },
                Coord { x: 3, y: 1 }
            ]
        );

        let vertical = Line {
            a: Coord { x: 1, y: 1 },
            b: Coord { x: 1, y: 3 },
        };

        assert_eq!(
            vertical.produce_covered_coords(false),
            vec![
                Coord { x: 1, y: 1 },
                Coord { x: 1, y: 2 },
                Coord { x: 1, y: 3 }
            ]
        );

        let diagonal_down = Line {
            a: Coord { x: 1, y: 1 },
            b: Coord { x: 3, y: 3 },
        };

        assert_eq!(diagonal_down.produce_covered_coords(false), vec![]);
        assert_eq!(
            diagonal_down.produce_covered_coords(true),
            vec![
                Coord { x: 1, y: 1 },
                Coord { x: 2, y: 2 },
                Coord { x: 3, y: 3 }
            ]
        );

        let diagonal_up = Line {
            a: Coord { x: 3, y: 1 },
            b: Coord { x: 1, y: 3 },
        };

        assert_eq!(diagonal_up.produce_covered_coords(false), vec![]);
        assert_eq!(
            diagonal_up.produce_covered_coords(true),
            vec![
                Coord { x: 1, y: 3 },
                Coord { x: 2, y: 2 },
                Coord { x: 3, y: 1 },
            ]
        );
    }
}

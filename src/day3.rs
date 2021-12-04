use std::env;
use std::process;

use common::load_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day3 <file>");
        process::exit(1);
    }

    let binary_rows: Vec<Vec<u8>> = load_file(&args[1])
        .into_iter()
        .map(|row| {
            row.as_bytes()
                .into_iter()
                .map(|c| match c {
                    48 => 0,
                    49 => 1,
                    _ => panic!("invalid data"),
                })
                .collect::<Vec<u8>>()
        })
        .collect();

    part1(&binary_rows);
    part2(&binary_rows);
}

fn part1(rows: &Vec<Vec<u8>>) -> i64 {
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();
    let width = rows[0].len();

    for i in 0..width {
        let res = most_common(rows, i);
        gamma_rate.push(res);
        epsilon_rate.push(if res == 1 { 0 } else { 1 });
    }

    let gamma_rate_dec: i64 = binary_to_dec(gamma_rate);
    let epsilon_rate_dec: i64 = binary_to_dec(epsilon_rate);

    println!("{:}", gamma_rate_dec * &epsilon_rate_dec);

    gamma_rate_dec * epsilon_rate_dec
}

fn part2(rows: &Vec<Vec<u8>>) {
    let oxygen = binary_to_dec(search(rows, false));
    let co2 = binary_to_dec(search(rows, true));
    println!("{:}", oxygen * co2);
}

fn search(rows: &Vec<Vec<u8>>, inverse: bool) -> Vec<u8> {
    let width = rows[0].len();

    let mut r = rows.to_owned();
    for i in 0..width {
        let needle = if inverse {
            if most_common(&r, i) == 1 {
                0
            } else {
                1
            }
        } else {
            most_common(&r, i)
        };
        r = filter_by(r.clone(), needle, i);
        if r.len() == 1 {
            return r[0].clone();
        }
    }
    panic!("Not found")
}

fn filter_by(rows: Vec<Vec<u8>>, value: u8, position: usize) -> Vec<Vec<u8>> {
    rows.into_iter()
        .filter(|row| row[position] == value)
        .collect()
}

fn binary_to_dec(bin: Vec<u8>) -> i64 {
    let width = bin.len() - 1;
    bin.into_iter()
        .enumerate()
        .map(|(i, r)| (r as i64) * (1 << (width - i)))
        .sum()
}

fn most_common(rows: &Vec<Vec<u8>>, position: usize) -> u8 {
    let mut counts = vec![0; 2];
    for r in rows {
        counts[r[position] as usize] += 1;
    }

    if counts[0] > counts[1] {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = vec![vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        // 001  * 110 = 1 * 6
        let res = part1(&data);

        assert_eq!(res, 6);
    }

    #[test]
    fn test_binary_to_dec() {
        let input = vec![0, 0, 0];
        assert_eq!(binary_to_dec(input), 0);

        let input = vec![0, 0, 1];
        assert_eq!(binary_to_dec(input), 1);

        let input = vec![0, 1, 0];
        assert_eq!(binary_to_dec(input), 2);

        let input = vec![1, 0, 0];
        assert_eq!(binary_to_dec(input), 4);

        let input = vec![1, 1, 0];
        assert_eq!(binary_to_dec(input), 6);

        let input = vec![1, 1, 1];
        assert_eq!(binary_to_dec(input), 7);
    }
}

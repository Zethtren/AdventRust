use rayon::prelude::*;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn check_puzzle(grid: Vec<Vec<char>>) -> Option<u128> {
    let len = grid.clone().len();
    let mid = len.div_ceil(2);
    let mut result = None;
    for idx in 1..len {
        let (start, interval) = if idx < mid {
            (0, idx)
        } else {
            (idx - (len - idx), len - idx)
        };
        let top: Vec<Vec<char>> = grid
            .clone()
            .into_iter()
            .skip(start)
            .take(interval)
            .collect();
        let bot: Vec<Vec<char>> = grid
            .clone()
            .into_iter()
            .skip(idx)
            .take(interval)
            .rev()
            .collect();
        if bot == top {
            result = Some(idx as u128);
        }
    }
    result
}

fn check_puzzle_off_by(grid: Vec<Vec<char>>, smudge: u128) -> Option<u128> {
    let len = grid.clone().len();
    let mid = len.div_ceil(2);
    let mut result = None;
    for idx in 1..len {
        let (start, interval) = if idx < mid {
            (0, idx)
        } else {
            (idx - (len - idx), len - idx)
        };
        let top: Vec<Vec<char>> = grid
            .clone()
            .into_iter()
            .skip(start)
            .take(interval)
            .collect();
        let bot: Vec<Vec<char>> = grid
            .clone()
            .into_iter()
            .skip(idx)
            .take(interval)
            .rev()
            .collect();
        if bot
            .iter()
            .flatten()
            .zip(top.iter().flatten())
            .filter(|(b, t)| b != t)
            .count() as u128
            == smudge
        {
            result = Some(idx as u128);
        }
    }
    result
}

fn input_grid(puzzle: &str) -> Vec<Vec<char>> {
    puzzle.lines().map(|line| line.chars().collect()).collect()
}

fn transpose_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let length = grid.clone().first().unwrap().len();
    (0..length)
        .map(|idx| grid.clone().into_iter().map(|line| line[idx]).collect())
        .collect()
}

fn check_horizontal(puzzle: Vec<Vec<char>>) -> Option<u128> {
    check_puzzle(puzzle)
}

fn check_vertical(puzzle: Vec<Vec<char>>) -> Option<u128> {
    check_puzzle(transpose_grid(puzzle))
}

fn check_symmetry(puzzle: Vec<Vec<char>>) -> Option<u128> {
    if let Some(reflection) = check_horizontal(puzzle.clone()) {
        Some(100 * reflection)
    } else {
        check_vertical(puzzle)
    }
}

fn check_horizontal_smudge(puzzle: Vec<Vec<char>>, smudge: u128) -> Option<u128> {
    check_puzzle_off_by(puzzle, smudge)
}

fn check_vertical_smudge(puzzle: Vec<Vec<char>>, smudge: u128) -> Option<u128> {
    check_puzzle_off_by(transpose_grid(puzzle), smudge)
}

fn check_symmetry_smudge(puzzle: Vec<Vec<char>>, smudge: u128) -> Option<u128> {
    if let Some(reflection) = check_horizontal_smudge(puzzle.clone(), smudge) {
        Some(100 * reflection)
    } else {
        check_vertical_smudge(puzzle, smudge)
    }
}

fn part_one(file: &str) -> u128 {
    file.split("\n\n")
        .par_bridge()
        .fold(
            || 0_u128,
            |acc, puzzle| {
                if let Some(val) = check_symmetry(input_grid(puzzle)) {
                    acc + val
                } else {
                    acc
                }
            },
        )
        .sum()
}

fn part_two(file: &str) -> u128 {
    file.split("\n\n")
        .par_bridge()
        .fold(
            || 0_u128,
            |acc, puzzle| {
                if let Some(val) = check_symmetry_smudge(input_grid(puzzle), 1) {
                    acc + val
                } else {
                    acc
                }
            },
        )
        .sum()
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let file = read_to_string(args.remove(0)).unwrap();
    let start = Instant::now();
    println!("Part 1: {} Elapsed: {:?}", part_one(&file), start.elapsed());
    let start = Instant::now();
    println!("Part 2: {} Elapsed: {:?}", part_two(&file), start.elapsed());
}

#[cfg(test)]
mod test {
    static TST: &str = "#.##..##.\n\
                        ..#.##.#.\n\
                        ##......#\n\
                        ##......#\n\
                        ..#.##.#.\n\
                        ..##..##.\n\
                        #.#.##.#.\n\
                                 \n\
                        #...##..#\n\
                        #....#..#\n\
                        ..##..###\n\
                        #####.##.\n\
                        #####.##.\n\
                        ..##..###\n\
                        #....#..#";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 405);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 400);
    }
}

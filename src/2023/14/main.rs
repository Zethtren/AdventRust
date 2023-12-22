use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn input_grid(puzzle: &str) -> Vec<Vec<char>> {
    puzzle.lines().map(|line| line.chars().collect()).collect()
}

fn transpose_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let length = grid.clone().first().unwrap().len();
    (0..length)
        .map(|idx| grid.clone().into_iter().map(|line| line[idx]).collect())
        .collect()
}

fn row_weight(row: Vec<char>) -> u128 {
    let true_top = row.len();
    let mut new_top = true_top;
    let mut count = 0;
    row.iter().zip(1..).for_each(|(c, i)| match c {
        '#' => new_top = true_top - i,
        'O' => {
            count += new_top;
            new_top -= 1;
        }
        _ => {}
    });
    count as u128
}

fn part_one(file: &str) -> u128 {
    transpose_grid(input_grid(file))
        .into_par_iter()
        .fold(|| 0_u128, |acc, line| acc + row_weight(line))
        .sum()
}

fn slide_up(row: Vec<char>) -> Vec<char> {
    let mut new_row = Vec::new();
    let mut current_block = 0;
    for (i, item) in row.into_iter().enumerate() {
        match item {
            'O' => new_row.insert(current_block, 'O'),
            '#' => {
                current_block = i + 1;
                new_row.insert(i, '#')
            }
            c => new_row.push(c),
        }
    }
    new_row
}

fn cycle_load(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let north = grid.into_par_iter().map(slide_up).collect();
    let west = transpose_grid(north)
        .into_par_iter()
        .map(slide_up)
        .collect();
    let south = transpose_grid(west)
        .into_par_iter()
        .map(|line| {
            slide_up(line.into_iter().rev().collect())
                .into_iter()
                .rev()
                .collect()
        })
        .collect();
    let east = transpose_grid(south)
        .into_par_iter()
        .map(|line| {
            slide_up(line.into_iter().rev().collect())
                .into_iter()
                .rev()
                .collect()
        })
        .collect();
    transpose_grid(east)
}

fn part_two(file: &str) -> u128 {
    let mut grid = transpose_grid(input_grid(file));

    let mut start = 0;
    let mut end = 0;
    let cap = 1_000_000_000;
    let mut count = 0;
    let mut set: HashMap<Vec<Vec<char>>, u128> = HashMap::new();
    let mut stop_cap = 0;

    while count < cap {
        grid = cycle_load(grid);
        count += 1;
        if let Some(&v) = set.get(&grid) {
            if (v == 2) && (start == 0) {
                start = count;
            } else if (v == 3) && (end == 0) {
                end = count;
            }
        }
        if (start > 0) && (end > 0) && (stop_cap == 0) {
            let length = end - start;
            stop_cap = cap - length;
            while count < stop_cap {
                count += length;
            }
        }
        *set.entry(grid.clone()).or_insert(0) += 1;
    }
    grid.into_par_iter()
        .fold(
            || 0_u128,
            |acc, line| {
                acc + line.iter().zip(0..).fold(0, |acc, c| {
                    if c.0 == &'O' {
                        acc + (line.len() - c.1) as u128
                    } else {
                        acc
                    }
                })
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
    static TST: &str = "O....#....\n\
                        O.OO#....#\n\
                        .....##...\n\
                        OO.#O....O\n\
                        .O.....O#.\n\
                        O.#..O.#.#\n\
                        ..O..#O..O\n\
                        .......O..\n\
                        #....###..\n\
                        #OO..#....";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 136);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 64);
    }
}

use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> i128 {
    let grid = file
        .lines()
        .map(|x| x.chars().map(|y| y == '@').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let x = grid.first().unwrap().len();
    let y = grid.len();
    (0..y)
        .map(|i| {
            (0..x)
                .map(|j| {
                    if grid[i][j] {
                        let valid_left = (j > 0);
                        let valid_right = ((j + 1) < x);
                        let valid_up = (i > 0);
                        let valid_down = ((i + 1) < y);
                        let check_left = valid_left && grid[i][j - 1];
                        let check_right = valid_right && grid[i][j + 1];
                        let check_up = valid_up && grid[i - 1][j];
                        let check_down = valid_down && grid[i + 1][j];
                        let check_left_up = valid_left && valid_up && grid[i - 1][j - 1];
                        let check_left_down = valid_left && valid_down && grid[i + 1][j - 1];
                        let check_right_up = valid_right && valid_up && grid[i - 1][j + 1];
                        let check_right_down = valid_right && valid_down && grid[i + 1][j + 1];
                        if vec![
                            check_up,
                            check_right_up,
                            check_left_up,
                            check_down,
                            check_left_down,
                            check_right_down,
                            check_left,
                            check_right,
                        ]
                        .iter()
                        .filter(|&&x| x)
                        .count()
                            < 4
                        {
                            return 1;
                        }
                    }
                    0
                })
                .sum::<i128>()
        })
        .sum()
}
fn part_two(file: &str) -> i128 {
    let mut grid = file
        .lines()
        .map(|x| x.chars().map(|y| y == '@').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let mut removed = 0;
    let mut newly_removed = 1;
    while newly_removed > 0 {
        let (new, new_grid) = remove_from_grid(grid);
        removed += new;
        grid = new_grid;
        newly_removed = new;
    }
    removed
}

fn remove_from_grid(grid: Vec<Vec<bool>>) -> (i128, Vec<Vec<bool>>) {
    let x = grid.first().unwrap().len();
    let y = grid.len();
    let mut new_grid = vec![vec![false; x]; y];
    let removed = (0..y)
        .map(|i| {
            (0..x)
                .map(|j| {
                    if grid[i][j] {
                        let valid_left = (j > 0);
                        let valid_right = ((j + 1) < x);
                        let valid_up = (i > 0);
                        let valid_down = ((i + 1) < y);
                        let check_left = valid_left && grid[i][j - 1];
                        let check_right = valid_right && grid[i][j + 1];
                        let check_up = valid_up && grid[i - 1][j];
                        let check_down = valid_down && grid[i + 1][j];
                        let check_left_up = valid_left && valid_up && grid[i - 1][j - 1];
                        let check_left_down = valid_left && valid_down && grid[i + 1][j - 1];
                        let check_right_up = valid_right && valid_up && grid[i - 1][j + 1];
                        let check_right_down = valid_right && valid_down && grid[i + 1][j + 1];
                        if vec![
                            check_up,
                            check_right_up,
                            check_left_up,
                            check_down,
                            check_left_down,
                            check_right_down,
                            check_left,
                            check_right,
                        ]
                        .iter()
                        .filter(|&&x| x)
                        .count()
                            < 4
                        {
                            return 1;
                        }
                        new_grid[i][j] = true
                    }
                    0
                })
                .sum::<i128>()
        })
        .sum();
    return (removed, new_grid);
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
    static TST: &str = "..@@.@@@@.\n\
                        @@@.@.@.@@\n\
                        @@@@@.@.@@\n\
                        @.@@@@..@.\n\
                        @@.@@@@.@@\n\
                        .@@@@@@@.@\n\
                        .@.@.@.@@@\n\
                        @.@@@.@@@@\n\
                        .@@@@@@@@.\n\
                        @.@.@@@.@.";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 6);
    }
}

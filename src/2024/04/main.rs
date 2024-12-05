use std::env;
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;


fn part_one(file: &str) -> i128 {
    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let x = grid[0].len();
    let y = grid.len();

    let left_bound = 3;
    let up_bound = 3;
    let right_bound = x - 4;
    let down_bound = y - 4;

    let mut count = 0;
    for i in 0..y {
        for j in 0..x {
            if grid[i][j] == 'X' {
                if (i >= up_bound) && (grid[i - 1][j] == 'M') && (grid[i - 2][j] == 'A') && (grid[i - 3][j] == 'S') {
                    count += 1;
                }
                if (i >= up_bound) && (j <= right_bound) && (grid[i - 1][j + 1] == 'M') && (grid[i - 2][j + 2] == 'A') && (grid[i - 3][j + 3] == 'S') {
                    count += 1;
                }
                if (j <= right_bound) && (grid[i][j + 1] == 'M') && (grid[i][j + 2] == 'A') && (grid[i][j + 3] == 'S') {
                    count += 1;
                }
                if (i <= down_bound) && (j <= right_bound) && (grid[i + 1][j + 1] == 'M') && (grid[i + 2][j + 2] == 'A') && (grid[i + 3][j + 3] == 'S') {
                    count += 1;
                }
                if (i <= down_bound) && (grid[i + 1][j] == 'M') && (grid[i + 2][j] == 'A') && (grid[i + 3][j] == 'S') {
                    count += 1;
                }
                if (i <= down_bound) && (j >= left_bound) && (grid[i + 1][j - 1] == 'M') && (grid[i + 2][j - 2] == 'A') && (grid[i + 3][j - 3] == 'S') {
                    count += 1;
                }
                if (j >= left_bound) && (grid[i][j - 1] == 'M') && (grid[i][j - 2] == 'A') && (grid[i][j - 3] == 'S') {
                    count += 1;
                }
                if (i >= up_bound) && (j >= left_bound) && (grid[i - 1][j - 1] == 'M') && (grid[i - 2][j - 2] == 'A') && (grid[i - 3][j - 3] == 'S') {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_two(file: &str) -> i128 {
    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let x = grid[0].len();
    let y = grid.len();

    let mut count = 0;
    for i in 1..(y-1) {
        for j in 1..(x-1) {
            if grid[i][j] == 'A' {
                if (((grid[i-1][j-1] == 'S') && (grid[i+1][j+1] == 'M')) || ((grid[i-1][j-1] == 'M') && (grid[i+1][j+1] == 'S'))) && (((grid[i-1][j+1] == 'S') && (grid[i+1][j-1] == 'M')) || ((grid[i-1][j+1] == 'M') && (grid[i+1][j-1] == 'S'))) {
                    count += 1;
                }
            }
        }
    }
    count
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
    static TST: &str = "MMMSXXMASM\n\
                        MSAMXMSMSA\n\
                        AMXSXMAAMM\n\
                        MSAMASMSMX\n\
                        XMASAMXAMM\n\
                        XXAMMXXAMA\n\
                        SMSMSASXSS\n\
                        SAXAMASAAA\n\
                        MAMMMXMMMM\n\
                        MXMXAXMASX";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 9);
    }
}



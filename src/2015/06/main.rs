use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> usize {
    let mut grid = vec![vec![0_usize; 1_000]; 1_000];
    file.lines().for_each(|x| {
        let mut parts = x.split(' ');
        let item = parts.next().expect("There is a part.");
        let pattern = {
            if item == "turn" {
                parts.next().expect("Need the action.")
            } else {
                item
            }
        };
        let mut starts = parts.next().expect("There is a part.").split(',');
        parts.next();
        let mut ends = parts.next().expect("There is a part.").split(',');
        let x = starts
            .next()
            .expect("x value")
            .parse::<u32>()
            .expect("Integer")
            ..=ends
                .next()
                .expect("x value")
                .parse::<u32>()
                .expect("Integer");

        let y = starts
            .next()
            .expect("y value")
            .parse::<u32>()
            .expect("Integer")
            ..=ends
                .next()
                .expect("y value")
                .parse::<u32>()
                .expect("Integer");

        match pattern {
            "on" => x.for_each(|i| y.clone().for_each(|j| grid[i as usize][j as usize] = 1)),
            "off" => x.for_each(|i| y.clone().for_each(|j| grid[i as usize][j as usize] = 0)),
            "toggle" => x.for_each(|i| {
                y.clone().for_each(|j| {
                    if grid[i as usize][j as usize] == 1 {
                        grid[i as usize][j as usize] = 0
                    } else {
                        grid[i as usize][j as usize] = 1
                    };
                });
            }),
            _ => (),
        }
    });
    grid.into_iter()
        .map(|x| {
            let sum: usize = x.into_iter().sum();
            sum
        })
        .sum()
}

fn part_two(file: &str) -> usize {
    let mut grid = vec![vec![0_usize; 1_000]; 1_000];
    file.lines().for_each(|x| {
        let mut parts = x.split(' ');
        let item = parts.next().expect("There is a part.");
        let pattern = {
            if item == "turn" {
                parts.next().expect("Need the action.")
            } else {
                item
            }
        };
        let mut starts = parts.next().expect("There is a part.").split(',');
        parts.next();
        let mut ends = parts.next().expect("There is a part.").split(',');
        let x = starts
            .next()
            .expect("x value")
            .parse::<u32>()
            .expect("Integer")
            ..=ends
                .next()
                .expect("x value")
                .parse::<u32>()
                .expect("Integer");

        let y = starts
            .next()
            .expect("y value")
            .parse::<u32>()
            .expect("Integer")
            ..=ends
                .next()
                .expect("y value")
                .parse::<u32>()
                .expect("Integer");

        match pattern {
            "on" => x.for_each(|i| y.clone().for_each(|j| grid[i as usize][j as usize] += 1)),
            "off" => x.for_each(|i| {
                y.clone().for_each(|j| {
                    if grid[i as usize][j as usize] != 0 {
                        grid[i as usize][j as usize] -= 1
                    }
                });
            }),
            "toggle" => x.for_each(|i| y.clone().for_each(|j| grid[i as usize][j as usize] += 2)),
            _ => (),
        }
    });
    grid.into_iter()
        .map(|x| {
            let sum: usize = x.into_iter().sum();
            sum
        })
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
    const TEST: &str = "\
        turn on 0,0 through 999,999\n\
        toggle 0,0 through 999,0\n\
        turn off 499,499 through 500,500
    ";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 998_996);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST), 5);
    }
}

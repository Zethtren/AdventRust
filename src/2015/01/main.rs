use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> i64 {
    file.lines()
        .map(|line| {
            line.chars().fold(0, |acc, c| {
                acc + match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                }
            })
        })
        .sum()
}

fn part_two(file: &str) -> i64 {
    let mut floor = 0;
    for line in file.lines() {
        for (c, i) in line.chars().zip(1..) {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => {}
            }
            if floor < 0 {
                return i;
            }
        }
    }
    -1
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
    const TEST: &str = "(())";
    const TEST2: &str = "()())";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST2), 5);
    }
}

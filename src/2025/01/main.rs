use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> i128 {
    let mut start = 50;
    file.lines()
        .map(|x| {
            let item = x.split_at(1);
            let dir = item.0;
            let count = item.1.parse::<i64>().unwrap();
            if dir == "R" {
                start += count;
                while start > 99 {
                    start -= 100;
                }
            }
            if dir == "L" {
                start -= count;
                while start < 0 {
                    start += 100;
                }
            }
            if start == 0 {
                return 1;
            }
            0
        })
        .sum()
}
fn part_two(file: &str) -> i64 {
    let mut start = 50;
    let mut count_zero = 0;
    file.lines().for_each(|x| {
        let item = x.split_at(1);
        let dir = item.0;
        let count = item.1.parse::<i64>().unwrap();
        if dir == "R" {
            start += count;
            while start > 99 {
                start -= 100;
                count_zero += 1;
            }
        }
        if dir == "L" {
            if start == 0 {
                count_zero -= 1;
            }
            start -= count;
            while start < 0 {
                start += 100;
                count_zero += 1;
            }
            if start == 0 {
                count_zero += 1;
            }
        }
    });
    count_zero
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
    static TST: &str = "L68
                        L30\n\
                        R48\n\
                        L5\n\
                        R60\n\
                        L55\n\
                        L1\n\
                        L99\n\
                        R14\n\
                        L82";

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

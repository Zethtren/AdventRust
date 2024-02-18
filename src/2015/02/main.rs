use itertools::Itertools;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> i64 {
    file.lines()
        .map(|line| {
            let sides: Vec<i64> = line
                .split('x')
                .take(3)
                .map(|c| c.parse().expect("Will parse"))
                .collect();
            let (sum, min) =
                sides
                    .into_iter()
                    .combinations(2)
                    .fold((0, i64::MAX), |(acc, min), i| {
                        let x = i[0] * i[1];
                        (acc + x, min.min(x))
                    });
            sum * 2 + min
        })
        .sum()
}

fn part_two(file: &str) -> i64 {
    file.lines()
        .map(|line| {
            let mut sides: Vec<i64> = line
                .split('x')
                .take(3)
                .map(|c| c.parse().expect("Will parse"))
                .collect::<Vec<i64>>();
            sides.sort();
            (sides[0] + sides[1]) * 2 + sides.into_iter().fold(1, |acc, x| acc * x)
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
    const TEST: &str = "2x3x4\n\
        1x1x10";
    const TEST2: &str = "()())";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 101);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST2), 5);
    }
}

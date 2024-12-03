use std::env;
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    static ref mul_regex: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Should be valid.");
}

fn part_one(file: &str) -> i128 {
    let data = file.replace('\n', "");
    mul_regex.captures_iter(&data).map(|c| c.extract()).map(|(_, [left, right])| left.parse::<i128>().expect("Won't fail") * right.parse::<i128>().expect("Won't fail") ).sum()
}

fn part_two(file: &str) -> i128 {
    0
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
    static TST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 161);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 4);
    }
}



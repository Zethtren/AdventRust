use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> i64 {
    file.lines()
        .map(|line| {
            let mut x = 0;
            loop {
                x += 1;
                let hashed = format!("{:x}", md5::compute(format!("{}{}", line, x)));
                if hashed.chars().take(5).all(|x| x == '0') {
                    break;
                }
            }
            x
        })
        .sum()
}

fn part_two(file: &str) -> usize {
    file.lines()
        .map(|line| {
            let mut x = 0;
            loop {
                x += 1;
                let hashed = format!("{:x}", md5::compute(format!("{}{}", line, x)));
                if hashed.chars().take(6).all(|x| x == '0') {
                    break;
                }
            }
            x
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
    const TEST: &str = "abcdef";
    const TEST2: &str = "()())";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 609043);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST2), 5);
    }
}

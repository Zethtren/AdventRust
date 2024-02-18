use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> i64 {}

fn part_two(file: &str) -> i64 {}

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
    const TEST: &str = "ugknbfddgicrmopn\n\
        aaa\n\
        jchzalrnumimnmhp\n\
        haegwjzuvuyypxyu\n\
        dvszwmarrgswjxmb\n\
        ";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST), 5);
    }
}

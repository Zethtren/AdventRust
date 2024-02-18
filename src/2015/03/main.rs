use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut set = HashSet::new();
    set.insert((x, y));
    file.lines().for_each(|line| {
        line.chars().for_each(|c| {
            match c {
                '^' => x -= 1,
                'v' => x += 1,
                '<' => y -= 1,
                '>' => y += 1,
                _ => {}
            };
            set.insert((x, y));
        })
    });
    set.len()
}

fn part_two(file: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let (mut a, mut b) = (0, 0);
    let mut set = HashSet::new();
    set.insert((x, y));
    file.lines().for_each(|line| {
        line.chars().zip(1..).for_each(|(c, i)| {
            if i % 2 == 0 {
                match c {
                    '^' => x -= 1,
                    'v' => x += 1,
                    '<' => y -= 1,
                    '>' => y += 1,
                    _ => {}
                };
                set.insert((x, y));
            } else {
                match c {
                    '^' => a -= 1,
                    'v' => a += 1,
                    '<' => b -= 1,
                    '>' => b += 1,
                    _ => {}
                };
                set.insert((a, b));
            };
        })
    });
    set.len()
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
    const TEST: &str = ">";
    const TEST2: &str = "()())";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST2), 5);
    }
}

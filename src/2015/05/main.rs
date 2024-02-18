use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> i64 {
    file.lines()
        .map(|line| {
            let bad = ["ab", "cd", "pq", "xy"];
            if bad.into_iter().map(|item| line.contains(item)).any(|x| x) {
                0
            } else {
                let mut prev = '0';
                let mut vowels = 0;
                let mut matched = (false, false);
                line.chars()
                    .map(|c| {
                        let m = prev;
                        prev = c;

                        if !matched.0 && "aeiou".contains(c) {
                            vowels += 1;
                            if vowels == 3 {
                                matched.0 = true;
                            }
                        };
                        if !matched.1 && (c == m) {
                            matched.1 = true;
                        }
                        if matched.0 && matched.1 {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<i64>()
                    .min(1)
            }
        })
        .sum()
}

fn part_two(file: &str) -> i64 {
    file.lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let mut matched = (false, false);
            let vals: Vec<_> = line.chars().collect();
            for i in 0..(vals.len() - 2) {
                if !matched.0 {
                    for j in (i + 2)..(vals.len() - 1) {
                        if vals[i] == vals[j] && vals[i + 1] == vals[j + 1] {
                            matched.0 = true;
                        }
                    }
                }
                if !matched.1 && (vals[i] == vals[i + 2]) {
                    matched.1 = true;
                }
            }
            if matched.0 && matched.1 {
                1
            } else {
                0
            }
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

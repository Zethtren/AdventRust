use rayon::prelude::*;
use std::collections::BTreeSet;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> i128 {
    file.trim()
        .split(',')
        .par_bridge()
        .map(|x| x.split_once('-').unwrap())
        .map(|(low, upp)| {
            ((low.parse::<i128>().unwrap())..=(upp.parse::<i128>().unwrap()))
                .map(|x| {
                    let val = x.to_string();
                    let (lft, rgt) = val.split_at(val.len() / 2);
                    if lft == rgt {
                        return x;
                    }
                    0
                })
                .sum::<i128>()
        })
        .sum()
}
fn part_two(file: &str) -> i128 {
    file.trim()
        .split(',')
        .par_bridge()
        .map(|x| x.split_once('-').unwrap())
        .map(|(low, upp)| {
            ((low.parse::<i128>().unwrap())..=(upp.parse::<i128>().unwrap()))
                .map(|x| {
                    let mut checked = BTreeSet::new();
                    let vals = x.to_string().chars().collect::<Vec<char>>();
                    (1..=(vals.len() / 2))
                        .map(|i| {
                            let mut checks = vals.chunks(i);
                            let first = checks.next().unwrap();
                            if checks.all(|y| y == first) {
                                if checked.contains(&x) {
                                    return 0;
                                }
                                checked.insert(x);
                                return x;
                            }
                            0
                        })
                        .sum::<i128>()
                })
                .sum::<i128>()
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
    static TST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 1227775554);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 6);
    }
}

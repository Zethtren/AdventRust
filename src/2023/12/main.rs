use itertools::{repeat_n, Itertools};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;
use std::{env, usize};

#[derive(Debug)]
struct Schema {
    springs: String,
    sequences: Vec<u128>,
    cache: HashMap<(String, Vec<u128>), u128>,
}

impl Schema {
    fn fold(line: &str, folds: usize) -> String {
        let mut parts: Vec<&str> = line.split(' ').collect();
        let base_springs = parts.remove(0);
        let base_sequences = parts.remove(0);
        let new_springs = repeat_n(base_springs, folds).join("?");
        let new_sequences = repeat_n(base_sequences, folds).join(",");
        format!("{} {}", new_springs, new_sequences)
    }

    fn count(&mut self, cfg: String, nums: Vec<u128>) -> u128 {
        if cfg.is_empty() {
            if nums.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }
        if nums.is_empty() {
            if cfg.contains('#') {
                return 0;
            } else {
                return 1;
            }
        }

        let key = (cfg.clone(), nums.clone());

        if let Some(&cached) = self.cache.get(&key) {
            return cached;
        }
        let mut result = 0;

        if ".?".contains(cfg.clone().chars().next().unwrap()) {
            result += self.count(
                cfg.clone().chars().skip(1).collect::<String>(),
                nums.clone(),
            );
        }
        if "#?".contains(cfg.clone().chars().next().unwrap()) {
            if (nums[0] <= cfg.len() as u128)
                && cfg.clone().chars().take(nums[0] as usize).all(|x| x != '.')
                && (nums[0] == cfg.len() as u128
                    || cfg
                        .clone()
                        .chars()
                        .take(nums[0] as usize + 1)
                        .last()
                        .unwrap()
                        != '#')
            {
                result += self.count(
                    cfg.chars().skip(nums[0] as usize + 1).collect::<String>(),
                    nums.into_iter().skip(1).collect(),
                );
            }
        }
        self.cache.insert(key, result);
        result
    }

    fn run(&mut self) -> u128 {
        self.count(self.springs.clone(), self.sequences.clone())
    }
}

impl From<&str> for Schema {
    fn from(input: &str) -> Schema {
        let vals: Vec<&str> = input.split(' ').collect();
        Schema {
            springs: vals.first().unwrap().to_string(),
            sequences: vals
                .last()
                .unwrap()
                .split(',')
                .map(|i| i.parse::<u128>().unwrap())
                .collect(),
            cache: HashMap::new(),
        }
    }
}

#[tracing::instrument]
fn part_one(file: &str) -> u128 {
    file.par_lines()
        .map(|springs| springs.into())
        .map(|mut schema: Schema| schema.run())
        .sum()
}

#[tracing::instrument]
fn part_two(file: &str) -> u128 {
    file.par_lines()
        .map(|line| Schema::fold(line, 5))
        .map(|springs| springs.as_str().into())
        .map(|mut schema: Schema| schema.run())
        .sum()
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let file = read_to_string(args.remove(0)).unwrap();
    let start = Instant::now();
    let res1 = part_one(&file);
    println!("Part 1: {} Elapsed: {:?}", res1, start.elapsed());
    let start = Instant::now();
    let res2 = part_two(&file);
    println!("Part 2: {} Elapsed: {:?}", res2, start.elapsed());
}

#[cfg(test)]
mod test {
    static TEST: &str = "???.### 1,1,3\n\
                        .??..??...?##. 1,1,3\n\
                        ?#?#?#?#?#?#?#? 1,3,1,6\n\
                        ????.#...#... 4,1,1\n\
                        ????.######..#####. 1,6,5\n\
                        ?###???????? 3,2,1";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST), 525152);
    }
}

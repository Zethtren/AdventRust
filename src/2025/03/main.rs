use itertools::max;
use rayon::prelude::*;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

const MAX_DEPTH: usize = 12;
const BASE: i128 = 10;

fn part_one(file: &str) -> u32 {
    file.lines()
        .par_bridge()
        .map(|x| {
            let nums = x
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let mut cur_max = 0;
            let len = nums.len();
            let mut index_subtractor: usize = 0;
            while index_subtractor < len {
                let start = nums.get(index_subtractor).unwrap() * 10;
                for i in (index_subtractor + 1)..len {
                    let cur = start + nums.get(i).unwrap();
                    cur_max = cur_max.max(cur);
                }
                index_subtractor += 1
            }
            cur_max
        })
        .sum()
}
fn part_two(file: &str) -> i128 {
    file.lines()
        .par_bridge()
        .map(|x| {
            let nums = x
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let max_array = recurse_array(0, 0, &nums);
            let mut joltage: i128 = 0;
            println!("{:?}", max_array);
            for x in 0..MAX_DEPTH {
                let current_val =
                    *max_array.get(x).unwrap() as i128 * (BASE.pow((MAX_DEPTH - x) as u32));
                joltage += current_val;
            }
            joltage / 10
        })
        .sum()
}

fn recurse_array(depth: usize, current_pos: usize, array: &Vec<u32>) -> Vec<u32> {
    // Start with len - MAX_DEPTH and find max. remove everything up to and including the index and
    // then repeat until len = current-length
    let current_length = MAX_DEPTH - (depth + 1);
    let current_slice = &array[current_pos..(array.len() - current_length)];
    // println!("{:?}", current_slice);
    // if current_slice.len() == current_length + 1 {
    //     return current_slice.to_vec();
    // }
    let current_max = current_slice.iter().max().unwrap();
    if (depth + 1) != MAX_DEPTH {
        let new_pos = array[current_pos..]
            .iter()
            .position(|x| x == current_max)
            .unwrap()
            + current_pos;
        let mut stack = vec![current_max.to_owned()];
        let end = recurse_array(depth + 1, new_pos + 1, array);
        // println!("{:?}, depth: {}, current_pos: {}", end, depth, new_pos + 1);
        stack.extend(end);
        return stack;
    }
    vec![current_max.to_owned()]
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
    static TST: &str = "987654321111111\n\
                        811111111111119\n\
                        234234234234278\n\
                        818181911112111";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 357);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 3121910778619);
    }
}

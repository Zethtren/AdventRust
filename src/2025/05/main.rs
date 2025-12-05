use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::time::Instant;

fn part_one(file: &str) -> i128 {
    let mut switch = false;
    let mut ranges: Vec<RangeInclusive<i128>> = Vec::new();
    let mut total = 0;
    file.lines().for_each(|line| {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            switch = true;
        } else if switch {
            let val = trimmed.parse::<i128>().unwrap();
            for range in ranges.clone() {
                let contains = range.contains(&val);
                if contains {
                    total += 1;
                    break;
                }
            }
        } else {
            if let Some((strt, end)) = line.split_once('-') {
                let st = strt.parse::<i128>().unwrap();
                let en = end.parse::<i128>().unwrap();
                ranges.push(st..=en);
            }
        }
    });
    total
}
fn part_two(file: &str) -> i128 {
    let mut switch = true;
    let mut ranges: Vec<(i128, i128)> = Vec::new();
    let mut collapsed_ranges: Vec<(i128, i128)> = Vec::new();
    let mut total = 0;
    file.lines().for_each(|line| {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            switch = false;
        } else if switch {
            if let Some((strt, end)) = line.split_once('-') {
                let st = strt.parse::<i128>().unwrap();
                let ed = end.parse::<i128>().unwrap();
                ranges.push((st, ed));
            }
        }
    });
    // Sort Vec on strts from low to high.
    ranges.sort_by(|(a, b), (c, d)| a.cmp(c));
    let mut rng = ranges.iter();
    // I need to pull the first value then compare it to each remaining. If any starts are within
    let (mut start, mut end) = rng.next().unwrap();
    while let Some((new_start, new_end)) = rng.next() {
        if (start..=end).contains(&new_start) {
            end = end.max(*new_end);
        } else {
            collapsed_ranges.push((start, end));
            start = *new_start;
            end = *new_end;
        }
    }
    collapsed_ranges.push((start, end));
    collapsed_ranges.into_iter().for_each(|(str, end)| {
        total += (end - str + 1);
    });
    total
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
    static TST: &str = "..@@.@@@@.\n\
                        @@@.@.@.@@\n\
                        @@@@@.@.@@\n\
                        @.@@@@..@.\n\
                        @@.@@@@.@@\n\
                        .@@@@@@@.@\n\
                        .@.@.@.@@@\n\
                        @.@@@.@@@@\n\
                        .@@@@@@@@.\n\
                        @.@.@@@.@.";

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

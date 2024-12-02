use std::env;
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;

fn check_floors(floors: Vec<i128>) -> i128 {
    let diffs: Vec<i128> = floors.split_last().unwrap().1.into_iter().zip(floors.split_first().unwrap().1).map(|(x, y)| x - y).collect();
    if diffs.iter().all(|&diff| (1 <= diff) & (diff <= 3)) {
        1
    } else if diffs.iter().all(|&diff| (-1 >= diff) & (diff >= -3)) {
        1
    } else {
        0
    }
}

fn check_floors_len(floors: Vec<i128>, len: usize) -> i128 {
    if check_floors(floors.clone()) == 0 {
        for i in 0..len {
            let mut new_floors = floors.clone();
            new_floors.remove(i);
            if check_floors(new_floors) == 1 {
                return 1
            } else {
                continue
            }
        }
        return 0
    }
    1
}

fn part_one(file: &str) -> i128 {
    file.lines().map(|line| {
        let floors: Vec<i128>  = line.split(' ').map(|x| x.parse::<i128>().expect("These should all be valid.")).collect();
        check_floors(floors)
    }).sum()
}

fn part_two(file: &str) -> i128 {
    file.lines().map(|line| {
        let floors: Vec<i128>  = line.split(' ').map(|x| x.parse::<i128>().expect("These should all be valid.")).collect();
        let checks = floors.len();
        check_floors_len(floors, checks)
    }).sum()
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
    static TST: &str = "7 6 4 2 1\n\
                        1 2 7 8 9\n\
                        9 7 6 2 1\n\
                        1 3 2 4 5\n\
                        8 6 4 4 1\n\
                        1 3 6 7 9";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 4);
    }
}



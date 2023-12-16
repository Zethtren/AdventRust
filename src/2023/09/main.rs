use std::{env, fs::read_to_string};

fn cascading_diffs(input: &mut Vec<i64>) -> Vec<i64> {
    let mut diffs = input[1..]
        .iter()
        .enumerate()
        .map(|(i, val)| val - input[i])
        .collect::<Vec<i64>>();
    if diffs.iter().all(|&x| x == 0) {
        diffs.push(0);
        diffs
    } else {
        cascading_diffs(&mut diffs);
        let new_end = diffs.last().unwrap() + input.last().unwrap();
        input.push(new_end);
        input.clone()
    }
}

fn cascading_front(input: &mut Vec<i64>) -> Vec<i64> {
    let mut diffs = input[..(input.len() - 1)]
        .iter()
        .enumerate()
        .map(|(i, val)| input[i + 1] - val)
        .collect::<Vec<i64>>();
    if diffs.iter().all(|&x| x == 0) {
        diffs.insert(0, 0);
        diffs
    } else {
        cascading_front(&mut diffs);
        let new_front = input.first().unwrap() - diffs.first().unwrap();
        input.insert(0, new_front);
        // println!("{:?}", input);
        input.clone()
    }
}

fn part_one(input: &str) -> i64 {
    let mut running_total = 0;
    for line in input.lines() {
        let mut vals = line
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let top_line = cascading_diffs(&mut vals);
        running_total += top_line.last().unwrap();
    }
    running_total
}

fn part_two(input: &str) -> i64 {
    let mut running_total = 0;
    for line in input.lines() {
        let mut vals = line
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let top_line = cascading_front(&mut vals);
        running_total += top_line.first().unwrap();
    }
    running_total
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let file = read_to_string(args.remove(0)).unwrap();
    let res1 = part_one(&file);
    println!("Part 1: {}", res1);
    let res2 = part_two(&file);
    println!("Part 2: {}", res2);
}

#[cfg(test)]
mod test {
    static TEST: &str = "0 3 6 9 12 15\n\
                         1 3 6 10 15 21\n\
                         10 13 16 21 30 45";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST), 2);
    }
}

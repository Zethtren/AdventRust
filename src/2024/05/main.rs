use std::env;
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;
use std::cmp::Ordering;


fn part_one(file: &str) -> i128 {
    let mut orders: Vec<(&str, &str)> = Vec::new();
    let mut sum = 0;
    file.lines().for_each(|line| {
        if line.contains('|') {
            let mut rule_items = line.split('|');
            orders.push((rule_items.next().expect("First Item"), rule_items.next().expect("Second Item.")));
        };
        // if line.is_empty() {
        //     // Create sort list from rules
        // };
        if line.contains(',') {
            let page_grouping: Vec<&str> = line.split(',').collect(); 
            let mut count = true;
            for (left, right) in &orders {
                if let Some(x) = page_grouping.iter().position(|x| x == left) {
                    if let Some(y) = page_grouping.iter().position(|x| x == right) {
                        if x > y {
                            count = false;
                        }
                    }
                }
            } 
            if count {
                sum += page_grouping[(page_grouping.len() - 1) / 2].parse::<i128>().expect("Will parse") 
            }
        };
    });
    sum
}

fn part_two(file: &str) -> i128 {
    let mut orders: Vec<(&str, &str)> = Vec::new();
    let mut sum = 0;
    let mut map: HashMap<&str, i32> = HashMap::new();
    file.lines().for_each(|line| {
        if line.contains('|') {
            let mut rule_items = line.split('|');
            orders.push((rule_items.next().expect("First Item"), rule_items.next().expect("Second Item.")));
        };
        // if line.is_empty() {
        //     for (left, right) in &orders {
        //         *map.entry(left).or_default() += 1;
        //         *map.entry(right).or_default() += 1;
        //     };
        //     for (_, right) in &orders {
        //         let _ = map.entry(right).or_default();
        //     };
        //     println!("{:#?}", map);
        // };
        if line.contains(',') {
            let mut page_grouping: Vec<&str> = line.split(',').collect(); 
            let mut count = true;
            for (left, right) in &orders {
                if let Some(x) = page_grouping.iter().position(|x| x == left) {
                    if let Some(y) = page_grouping.iter().position(|x| x == right) {
                        if x > y {
                            count = false;
                        }
                    }
                }
            } 
            if !count {
                page_grouping.sort_by(|a, b| {
                    for (left, right) in &orders {
                        if (left == a) && (right == b) {
                            return Ordering::Less
                        } 
                        if (left == b) && (right == a) {
                            return Ordering::Greater
                        }
                    };
                    return Ordering::Equal
                });
                sum += page_grouping[(page_grouping.len() - 1) / 2].parse::<i128>().expect("Will parse") 
            }
        };
    });
    sum
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
    static TST: &str = "47|53\n\
                        97|13\n\
                        97|61\n\
                        97|47\n\
                        75|29\n\
                        61|13\n\
                        75|53\n\
                        29|13\n\
                        97|29\n\
                        53|29\n\
                        61|53\n\
                        97|53\n\
                        61|29\n\
                        47|13\n\
                        75|47\n\
                        97|75\n\
                        47|61\n\
                        75|61\n\
                        47|29\n\
                        75|13\n\
                        53|13\n\
                        \n\
                        75,47,61,53,29\n\
                        97,61,53,29,13\n\
                        75,29,13\n\
                        75,97,47,61,53\n\
                        61,13,29\n\
                        97,13,75,29,47";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 143);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 123);
    }
}



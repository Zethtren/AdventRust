use std::env;
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;

fn part_one(file: &str) -> i128 {
    let (mut left, mut right): (Vec<i128>, Vec<i128>) = file.lines().map(|x| {
        let mut items = x.split("   ");
        let left = items.next().unwrap().parse::<i128>().unwrap();
        let right = items.next().unwrap().parse::<i128>().unwrap();
        (left, right)
    }).unzip(); //.collect::<vec<(i128, i128)>>().unzip()
    left.sort();
    right.sort();
    left.into_iter().zip(right).map(|(x, y)| (x - y).abs()).sum() 
}
fn part_two(file: &str) -> i128 {
    let mut left: HashMap<i128, i128> = HashMap::new();
    let mut right: HashMap<i128, i128> = HashMap::new();
    file.lines().for_each(|x| {
        let mut items = x.split("   ");
        let y = items.next().unwrap().parse::<i128>().unwrap();
        let z = items.next().unwrap().parse::<i128>().unwrap();
        *left.entry(y).or_default() += 1;
        *right.entry(z).or_default() += 1;
    });
    left.into_iter().map(|(key, count)| {
        let x = right.get(&key);
        if x.is_some() {
            key * count * x.expect("Already confirmed its some.")
        } else {
            0
        }
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
    static TST: &str = "3   4\n\
                        4   3\n\
                        2   5\n\
                        1   3\n\
                        3   9\n\
                        3   3";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 31);
    }
}


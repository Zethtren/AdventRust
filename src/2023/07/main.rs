use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;

use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rules {
    Normal,
    Joker,
    Joker2,
}

fn compare_cards(a: &str, b: &str, rules: Rules) -> Ordering {
    let mut a_map: HashMap<char, u32> = HashMap::new();
    a.chars().for_each(|c| *a_map.entry(c).or_insert(0) += 1);

    let mut b_map: HashMap<char, u32> = HashMap::new();
    b.chars().for_each(|c| *b_map.entry(c).or_insert(0) += 1);

    if rules == Rules::Joker {
        if let Some(a_j) = a_map.remove(&'J') {
            if let Some((&a_big, _)) = a_map.iter().sorted_by(|a, b| b.1.cmp(a.1)).next() {
                *a_map.entry(a_big).or_insert(0) += a_j;
            }
        }
        if let Some(b_j) = b_map.remove(&'J') {
            if let Some((&b_big, _)) = b_map.iter().sorted_by(|a, b| b.1.cmp(a.1)).next() {
                *b_map.entry(b_big).or_insert(0) += b_j;
            }
        }
    }

    let mut sorting = Ordering::Equal;
    let hand_type = a_map
        .iter()
        .sorted_by(|a, b| b.1.cmp(a.1))
        .zip(b_map.iter().sorted_by(|a, b| b.1.cmp(a.1)));
    for ((_, a), (_, b)) in hand_type {
        match a.cmp(b) {
            Ordering::Equal => {
                continue;
            }
            order => {
                sorting = order;
                break;
            }
        }
    }
    let cards = match rules {
        Rules::Joker | Rules::Joker2 => "AKQT98765432J",
        Rules::Normal => "AKQJT98765432",
    };

    if sorting == Ordering::Equal {
        if rules == Rules::Joker {
            sorting = compare_cards(a, b, Rules::Joker2)
        };
        for (a, b) in a.chars().zip(b.chars()) {
            match cards.find(b).unwrap().cmp(&cards.find(a).unwrap()) {
                Ordering::Equal => {
                    continue;
                }
                order => {
                    sorting = order;
                    break;
                }
            }
        }
    }
    sorting
}

fn get_hands(input: &str) -> Vec<(&str, u64)> {
    input
        .lines()
        .map(|line| line.split(' ').take(2))
        .map(|mut line| {
            (
                line.next().unwrap(),
                line.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect()
}

fn part_one(input: &str) -> u64 {
    let mut hands = get_hands(input);
    hands.sort_by(|a, b| compare_cards(a.0, b.0, Rules::Normal));

    hands
        .iter()
        .zip(1..)
        .fold(0, |acc, (hand, i)| acc + i * hand.1)
}
fn part_two(input: &str) -> u64 {
    let mut hands = get_hands(input);
    hands.sort_by(|a, b| compare_cards(a.0, b.0, Rules::Joker));

    hands
        .iter()
        .zip(1..)
        .fold(0, |acc, (hand, i)| acc + i * hand.1)
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let path = args.remove(0);
    let input = read_to_string(path).unwrap();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    static TEST: &str = "32T3K 765\n\
                        T55J5 684\n\
                        KK677 28\n\
                        KTJJT 220\n\
                        QQQJA 483";
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 6440);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST), 5905);
    }
}

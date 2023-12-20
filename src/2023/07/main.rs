use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;

use std::fs::read_to_string;
use std::time::Instant;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rules {
    Normal,
    Joker,
}

fn make_card_map(cards: &str, rule: Rules) -> HashMap<char, u64> {
    let mut map: HashMap<char, u64> = HashMap::new();
    cards.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    if rule == Rules::Joker {
        if let Some(j) = map.remove(&'J') {
            if let Some((&big, _)) = map.iter().sorted_by(|a, b| b.1.cmp(a.1)).next() {
                *map.entry(big).or_insert(0) += j;
            } else {
                map.insert('J', 5);
            }
        }
    }
    map
}

fn order_by_hand_type(a: &str, b: &str, rule: Rules) -> Ordering {
    let a_map = make_card_map(a, rule);
    let b_map = make_card_map(b, rule);

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
    sorting
}

fn compare_cards(a: &str, b: &str, rules: Rules) -> Ordering {
    let cards = match rules {
        Rules::Joker => "AKQT98765432J",
        Rules::Normal => "AKQJT98765432",
    };

    let mut sorting = order_by_hand_type(a, b, rules);

    if sorting == Ordering::Equal {
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
    if rules == Rules::Joker && sorting == Ordering::Equal {
        order_by_hand_type(a, b, Rules::Normal)
    } else {
        sorting
    }
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

    let mut start = Instant::now();
    println!(
        "Part 1: {}, Elapsed: {:?}",
        part_one(&input),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Part 2: {}, Elapsed: {:?}",
        part_two(&input),
        start.elapsed()
    );
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

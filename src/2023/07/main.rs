use std::collections::HashMap;
use std::env;
use std::{cmp, cmp::Ordering};

use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    None,
}

impl From<char> for Card {
    fn from(value: char) -> Card {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => Card::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum CardTwo {
    A,
    K,
    Q,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
    None,
}

impl From<char> for CardTwo {
    fn from(value: char) -> Self {
        match value {
            'A' => CardTwo::A,
            'K' => CardTwo::K,
            'Q' => CardTwo::Q,
            'J' => CardTwo::J,
            'T' => CardTwo::Ten,
            '9' => CardTwo::Nine,
            '8' => CardTwo::Eight,
            '7' => CardTwo::Seven,
            '6' => CardTwo::Six,
            '5' => CardTwo::Five,
            '4' => CardTwo::Four,
            '3' => CardTwo::Three,
            '2' => CardTwo::Two,
            _ => CardTwo::None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    card_one: Card,
    card_two: Card,
    card_three: Card,
    card_four: Card,
    card_five: Card,
    ante: u32,
}

impl Hand {
    fn new(input: &str) -> Hand {
        let mut items: Vec<&str> = input.trim().split(' ').collect();
        let cards = items.remove(0);
        let ante = items.remove(0).parse::<u32>().unwrap();
        let mut card_vec: Vec<char> = cards.chars().collect();
        Hand {
            card_one: card_vec.remove(0).into(),
            card_two: card_vec.remove(0).into(),
            card_three: card_vec.remove(0).into(),
            card_four: card_vec.remove(0).into(),
            card_five: card_vec.remove(0).into(),
            ante,
        }
    }

    fn as_array(&self) -> Vec<Card> {
        vec![
            self.card_one,
            self.card_two,
            self.card_three,
            self.card_four,
            self.card_five,
        ]
    }

    fn hand_type(&self) -> HandType {
        let mut map: HashMap<Card, u32> = HashMap::new();
        *map.entry(self.card_one).or_insert(0) += 1;
        *map.entry(self.card_two).or_insert(0) += 1;
        *map.entry(self.card_three).or_insert(0) += 1;
        *map.entry(self.card_five).or_insert(0) += 1;
        *map.entry(self.card_four).or_insert(0) += 1;
        let mut max_type: HandType = HandType::HighCard;
        for (_, count) in map {
            if count == 5 {
                max_type = HandType::FiveOfAKind;
            } else if count == 4 {
                max_type = HandType::FourOfAKind;
            } else if count == 3 {
                max_type = match max_type {
                    HandType::OnePair => HandType::FullHouse,
                    _ => HandType::ThreeOfAKind,
                }
            } else if count == 2 {
                max_type = match max_type {
                    HandType::ThreeOfAKind => HandType::FullHouse,
                    HandType::OnePair => HandType::TwoPair,
                    _ => HandType::OnePair,
                }
            } else {
                continue;
            }
        }
        max_type
    }
}

impl PartialEq for Hand {
    fn eq(self: &Hand, rhs: &Hand) -> bool {
        if (self.card_one == rhs.card_one)
            && (self.card_two == rhs.card_two)
            && (self.card_three == rhs.card_three)
            && (self.card_four == rhs.card_four)
            && (self.card_five == rhs.card_five)
        {
            return true;
        }
        false
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, rhs: &Hand) -> Ordering {
        if self == rhs {
            return Ordering::Equal;
        }
        match self.hand_type().cmp(&rhs.hand_type()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let lh = self.as_array();
                let rh = rhs.as_array();
                for (i, lc) in lh.iter().enumerate() {
                    match lc.cmp(&rh[i]) {
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                        Ordering::Equal => {
                            continue;
                        }
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct HandTwo {
    card_one: CardTwo,
    card_two: CardTwo,
    card_three: CardTwo,
    card_four: CardTwo,
    card_five: CardTwo,
    ante: u32,
}

impl HandTwo {
    fn new(input: &str) -> Self {
        let mut items: Vec<&str> = input.trim().split(' ').collect();
        let cards = items.remove(0);
        let ante = items.remove(0).parse::<u32>().unwrap();
        let mut card_vec: Vec<char> = cards.chars().collect();
        HandTwo {
            card_one: card_vec.remove(0).into(),
            card_two: card_vec.remove(0).into(),
            card_three: card_vec.remove(0).into(),
            card_four: card_vec.remove(0).into(),
            card_five: card_vec.remove(0).into(),
            ante,
        }
    }

    fn as_array(&self) -> Vec<CardTwo> {
        vec![
            self.card_one,
            self.card_two,
            self.card_three,
            self.card_four,
            self.card_five,
        ]
    }

    fn hand_type(&self) -> HandType {
        let mut map: HashMap<CardTwo, u32> = HashMap::new();
        *map.entry(self.card_one).or_insert(0) += 1;
        *map.entry(self.card_two).or_insert(0) += 1;
        *map.entry(self.card_three).or_insert(0) += 1;
        *map.entry(self.card_five).or_insert(0) += 1;
        *map.entry(self.card_four).or_insert(0) += 1;
        let mut max_type: HandType = HandType::HighCard;
        let j_count = if let Some(j_count) = map.remove(&CardTwo::from('J')) {
            j_count
        } else {
            0
        };
        unmapped
        for (card, count) in &map {
            match count + j_count {
                5 => {
                    max_type = cmp::max(HandType::FiveOfAKind, max_type);
                    break;
                }
                4 => {
                    max_type = cmp::max(HandType::FourOfAKind, max_type);
                    break;
                }
                3 => {
                    max_type = cmp::max(HandType::ThreeOfAKind, max_type);
                    break;
                }
                2 => {
                    max_type = cmp::max(HandType::OnePair, max_type);
                }
                _ => {
                    continue;
                }
            }
        }
        for (card, count) in map {
            match max_type {
                HandType::ThreeOfAKind => {
                    match count {
                        2 => {
                            max_type = HandType::FullHouse;
                            break;
                        }
                        _ => {
                            continue;
                        }
                    };
                }
                HandType::OnePair => {
                    match count {
                        3 => {
                            max_type = HandType::FullHouse;
                            break;
                        }
                        2 => {
                            max_type = HandType::TwoPair;
                            break;
                        }
                        _ => {
                            continue;
                        }
                    };
                }
                _ => {
                    max_type = HandType::HighCard;
                }
            }
        }

        max_type
    }
}

impl PartialEq for HandTwo {
    fn eq(self: &HandTwo, rhs: &HandTwo) -> bool {
        if (self.card_one == rhs.card_one)
            && (self.card_two == rhs.card_two)
            && (self.card_three == rhs.card_three)
            && (self.card_four == rhs.card_four)
            && (self.card_five == rhs.card_five)
        {
            return true;
        }
        false
    }
}

impl Eq for HandTwo {}

impl Ord for HandTwo {
    fn cmp(&self, rhs: &HandTwo) -> Ordering {
        if self == rhs {
            return Ordering::Equal;
        }
        match self.hand_type().cmp(&rhs.hand_type()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let lh = self.as_array();
                let rh = rhs.as_array();
                for (i, lc) in lh.iter().enumerate() {
                    match lc.cmp(&rh[i]) {
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                        Ordering::Equal => {
                            continue;
                        }
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for HandTwo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn new(input: &str) -> Game {
        let mut hands: Vec<Hand> = Vec::new();
        for line in input.lines() {
            hands.push(Hand::new(line))
        }
        println!("Hands: {:?}", hands);
        Game { hands }
    }
    fn sort_hands(&mut self) {
        self.hands.sort_by(|a, b| b.cmp(a));
    }
    fn play(&mut self) -> u32 {
        self.sort_hands();
        let mut total = 0;
        for (hand, i) in self.hands.iter().zip(0..) {
            total += (i + 1) * hand.ante
        }
        total
    }
}

struct GameTwo {
    hands: Vec<Hand>,
}

impl GameTwo {
    fn new(input: &str) -> GameTwo {
        let mut hands: Vec<Hand> = Vec::new();
        for line in input.lines() {
            hands.push(Hand::new(line))
        }
        println!("Hands: {:?}", hands);
        GameTwo { hands }
    }
    fn sort_hands(&mut self) {
        self.hands.sort_by(|a, b| b.cmp(a));
    }
    fn play(&mut self) -> u32 {
        self.sort_hands();
        let mut total = 0;
        for (hand, i) in self.hands.iter().zip(0..) {
            total += (i + 1) * hand.ante
        }
        total
    }
}

fn part_one(input: &str) -> u32 {
    let mut game = Game::new(input);
    game.play()
}

fn part_two(input: &str) -> u32 {
    let mut game = GameTwo::new(input);
    game.play()
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let path = args.remove(0);
    let input = read_to_string(path).unwrap();

    let res_one = part_one(&input);
    println!("Part 1: {}", res_one);

    let res_two = part_two(&input);
    println!("Part 2: {}", res_two);
}

#[cfg(test)]
mod test {
    static TEST: &str = "32T3K 765\
        T55J5 684\
        KK677 28\
        KTJJT 220\
        QQQJA 483";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 6440);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST), 0);
    }
}

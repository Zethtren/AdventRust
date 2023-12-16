use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::mem;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        a = mem::replace(&mut b, a);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

#[derive(Debug)]
struct Node<'a> {
    l: &'a str,
    r: &'a str,
}

impl<'a> Node<'a> {
    fn new_from_str_tuple(input: &'a str) -> Self {
        let mut vals = input.trim()[1..(input.len() - 1)]
            .split(',')
            .collect::<Vec<&'a str>>();
        Node {
            l: vals.remove(0).trim(),
            r: vals.remove(0).trim(),
        }
    }
}

struct Game<'a> {
    instructions: &'a str,
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Game<'a> {
    fn new(input: &'a str) -> Self {
        let mut instructions: Option<&'a str> = None;
        let mut nodes: HashMap<&'a str, Node<'a>> = HashMap::new();
        for line in input.lines() {
            if line.is_empty() {
                continue;
            } else if instructions.is_none() {
                instructions = Some(line);
            } else {
                let mut items = line.split('=').collect::<Vec<&'a str>>();
                let name = items.remove(0).trim();
                nodes.insert(name, Node::new_from_str_tuple(items.remove(0).trim()));
            }
        }
        Game {
            instructions: instructions.unwrap(),
            nodes,
        }
    }
    fn play(&self, start: &str, end: &str) -> u64 {
        let mut current = start;
        let mut steps = 0;

        for instruction in self.instructions.chars().into_iter().cycle() {
            match instruction {
                'L' => current = self.nodes.get(current).unwrap().l,
                'R' => current = self.nodes.get(current).unwrap().r,
                _ => {
                    panic!("Unknown instruction.")
                }
            }
            steps += 1;
            if current.ends_with(end) {
                break;
            }
        }
        steps
    }

    fn ghost_play(&self) -> u64 {
        let mut lengths: Vec<u64> = Vec::new();
        let mut starts: Vec<&str> = Vec::new();
        for (k, _) in &self.nodes {
            if k.ends_with('A') {
                starts.push(k)
            };
        }
        for start in starts {
            lengths.push(self.play(start, "Z"));
        }
        lengths.into_iter().reduce(|acc, a| lcm(acc, a)).unwrap()
    }
}

fn part_one(input: &str) -> u64 {
    let game = Game::new(input);
    game.play("AAA", "ZZZ")
}

fn part_two(input: &str) -> u64 {
    let game = Game::new(input);
    game.ghost_play()
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
    static TEST: &str = "RL\n\
        \n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)";

    static TEST2: &str = "LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 2);
        assert_eq!(part_one(TEST2), 6);
    }
}

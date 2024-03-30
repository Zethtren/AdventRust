use std::env;
use std::fs::read_to_string;
use std::time::Instant;

enum Actions {
    ASSIGN(u16),
    NOT(String),
    AND(String, String),
    OR(String, String),
    RSHIFT(String, String),
    LSHIFT(String, String),
    ANDK(String, u16),
    ORK(String, u16),
    RSHIFTK(String, u16),
    LSHIFTK(String, u16),
    ASSIGNK(String),
}

const DIGITS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

fn part_one(file: &str) -> usize {
    let mut map = std::collections::HashMap::new();
    file.lines().for_each(|x| {
        let mut parts = x.split(" -> ");
        let actions = parts.next().expect("Will be actions");
        let key = parts.next().expect("Will be a key.");
        let v = if actions.contains("NOT") {
            let key = actions.replace("NOT ", "");
            if key.contains(&DIGITS) {
                Actions::ASSIGN(!key.parse::<u16>().expect("Will Parse"))
            } else {
                Actions::NOT(key)
            };
        } else if actions.contains("AND") {
            let keys = 
        }
        map.insert(key, v);
    });
    0
}

fn part_two(file: &str) -> usize {
    0
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
    const TEST: &str = "\
        turn on 0,0 through 999,999\n\
        toggle 0,0 through 999,0\n\
        turn off 499,499 through 500,500
    ";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 998_996);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST), 5);
    }
}

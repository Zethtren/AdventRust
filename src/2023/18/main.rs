use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn parse_from_label(line: &str) -> (char, i128) {
    let mut parts = line.trim().split(' ');
    if let (Some(dir), Some(dis)) = (parts.next(), parts.next()) {
        if let (Some(direction), Ok(distance)) = (dir.chars().next(), dis.parse::<i128>()) {
            (direction, distance)
        } else {
            panic!("Bad Parse");
        }
    } else {
        panic!("Bad parse on line");
    }
}

fn next_point(dir: char, dist: i128, cur: (i128, i128)) -> (i128, i128) {
    match dir {
        'R' => (cur.0 + dist, cur.1),
        'L' => (cur.0 - dist, cur.1),
        'U' => (cur.0, cur.1 - dist),
        'D' => (cur.0, cur.1 + dist),
        _ => panic!("Impossible input."),
    }
}

fn shoelace_from_file(input: &str, parse_fn: &dyn Fn(&str) -> (char, i128)) -> i128 {
    let folded = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(parse_fn)
        .fold(((0, 0), 0, 0), |acc, point| {
            let np = next_point(point.0, point.1, acc.0);
            let shoelace = acc.0 .0 * np.1 - np.0 * acc.0 .1;
            (np, acc.1 + shoelace, acc.2 + point.1)
        });
    (folded.1.abs() + folded.2) / 2 + 1
}

fn parse_from_color(line: &str) -> (char, i128) {
    let items = line.trim().split(' ');
    if let Some(color) = items.last() {
        let mut col_str: String = color.chars().filter(|c| c.is_alphanumeric()).collect();
        let dir = match col_str.pop() {
            Some('0') => 'R',
            Some('1') => 'D',
            Some('2') => 'L',
            Some('3') => 'U',
            _ => panic!("Bad Dir"),
        };
        if let Ok(steps) = i128::from_str_radix(&col_str, 16) {
            (dir, steps)
        } else {
            panic!("Bad Steps")
        }
    } else {
        panic!("Bad Color")
    }
}

fn part_one(file: &str) -> i128 {
    shoelace_from_file(file, &parse_from_label)
}

fn part_two(file: &str) -> i128 {
    shoelace_from_file(file, &parse_from_color)
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
    static TST: &str = "R 6 (#70c710)\n\
                        D 5 (#0dc571)\n\
                        L 2 (#5713f0)\n\
                        D 2 (#d2c081)\n\
                        R 2 (#59c680)\n\
                        D 2 (#411b91)\n\
                        L 5 (#8ceee2)\n\
                        U 2 (#caa173)\n\
                        L 1 (#1b58a2)\n\
                        U 2 (#caa171)\n\
                        R 2 (#7807d2)\n\
                        U 3 (#a77fa3)\n\
                        L 2 (#015232)\n\
                        U 2 (#7a21e3)";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 62);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 952408144115);
    }
}


use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use std::time::Instant;

type Map<'a> = HashMap<&'a str, Vec<(Option<Xmas>, Option<Rule>, Option<u64>, &'a str)>>;

#[derive(Debug, Clone, Copy)]
struct Ranges {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

#[derive(Debug, Clone, Copy)]
enum Xmas {
    X,
    M,
    A,
    S,
}

impl From<&str> for Xmas {
    fn from(value: &str) -> Self {
        match value {
            "x" => Xmas::X,
            "m" => Xmas::M,
            "a" => Xmas::A,
            "s" => Xmas::S,
            _ => panic!("Needs to be x, m, a, or s"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rule {
    GT,
    LT,
}

fn part_one(file: &str) -> u64 {
    let map = make_map(file);
    file.lines()
        .filter(|line| !line.is_empty() && line.starts_with('{'))
        .map(|line| {
            let items = line.replace(['{', '}'], "");
            let mut items = items.split(',');
            let x = items
                .next()
                .expect("Will be x")
                .replace("x=", "")
                .parse::<u64>()
                .expect("Will parse to int");
            let m = items
                .next()
                .expect("Will be m")
                .replace("m=", "")
                .parse::<u64>()
                .expect("Will parse to int");
            let a = items
                .next()
                .expect("Will be a")
                .replace("a=", "")
                .parse::<u64>()
                .expect("Will parse to int");
            let s = items
                .next()
                .expect("Will be s")
                .replace("s=", "")
                .parse::<u64>()
                .expect("Will parse to int");
            let mut key = "in";
            let mut val = 0;
            loop {
                match key {
                    "A" => {
                        val = x + m + a + s;
                        break;
                    }
                    "R" => break,
                    _ => {
                        let check = map
                            .get(key.replace([')', '(', '}'], "").as_str())
                            .expect("There will be a key");
                        for i in check {
                            if let (Some(n), Some(c), Some(v)) = (i.0, i.1, i.2) {
                                let cmp_val = match n {
                                    Xmas::X => x,
                                    Xmas::M => m,
                                    Xmas::A => a,
                                    Xmas::S => s,
                                };
                                match c {
                                    Rule::GT => {
                                        if cmp_val > v {
                                            key = i.3;
                                            break;
                                        } else {
                                            continue;
                                        }
                                    }
                                    Rule::LT => {
                                        if cmp_val < v {
                                            key = i.3;
                                            break;
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                key = i.3;
                                break;
                            }
                        }
                    }
                }
            }
            val
        })
        .sum()
}

impl Ranges {
    fn check_possible(&self) -> bool {
        return ![self.x, self.m, self.a, self.s].iter().any(|i| i.0 > i.1);
    }
    fn split(&self, letter: Xmas, rule: Rule, val: u64) -> (Option<Ranges>, Option<Ranges>) {
        let update = match letter {
            Xmas::X => self.x,
            Xmas::M => self.m,
            Xmas::A => self.a,
            Xmas::S => self.s,
        };
        let (good_letter, bad_letter) = match rule {
            Rule::GT => {
                let mut good = update;
                let mut bad = update;
                bad.1 = bad.1.min(val);
                good.0 = good.0.max(val + 1);
                (good, bad)
            }
            Rule::LT => {
                let mut good = update;
                let mut bad = update;
                good.1 = good.1.min(val - 1);
                bad.0 = bad.0.max(val);
                (good, bad)
            }
        };
        let (mut good, mut bad): (Ranges, Ranges) = (*self, *self);
        match letter {
            Xmas::X => {
                good.x = good_letter;
                bad.x = bad_letter;
            }
            Xmas::M => {
                good.m = good_letter;
                bad.m = bad_letter;
            }
            Xmas::A => {
                good.a = good_letter;
                bad.a = bad_letter;
            }
            Xmas::S => {
                good.s = good_letter;
                bad.s = bad_letter;
            }
        };
        match (good.check_possible(), bad.check_possible()) {
            (true, true) => (Some(good), Some(bad)),
            (false, true) => (None, Some(bad)),
            (true, false) => (Some(good), None),
            (false, false) => (None, None),
        }
    }

    fn calculate(&mut self, map: Arc<Mutex<Map>>, key: &str) -> u64 {
        if key == "A" {
            return [self.x, self.m, self.a, self.s]
                .iter()
                .fold(1, |acc, a| acc * (a.1 - a.0 + 1));
        }
        if key == "R" {
            return 0;
        }
        let maps = map.lock().unwrap();
        let steps = maps.get(key).unwrap().clone();
        drop(maps);
        let mut total = 0;
        for step in steps {
            match step {
                (None, None, None, next_key) => {
                    total += self.calculate(Arc::clone(&map), next_key);
                }
                (Some(xmas), Some(rule), Some(val), next_key) => {
                    let (new, cont) = self.split(xmas, rule, val);
                    if let Some(mut new) = new {
                        total += new.calculate(Arc::clone(&map), next_key);
                    }
                    if let Some(cont) = cont {
                        *self = cont;
                    } else {
                        break;
                    }
                }
                _ => {}
            }
        }
        total
    }
}

impl Default for Ranges {
    fn default() -> Self {
        Ranges {
            x: (1, 4_000),
            m: (1, 4_000),
            a: (1, 4_000),
            s: (1, 4_000),
        }
    }
}

fn make_map(input: &str) -> Map {
    input
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('{'))
        .map(|line| {
            let mut first = line.split('{');
            let key = first.next().expect("There will be a key");
            let second = first.next().expect("There will be items");
            let vals: Vec<_> = second
                .split(',')
                .map(|mut item| {
                    item = item
                        .split('}')
                        .next()
                        .expect("This is the end delimiter it will exist");
                    if item.contains(':') {
                        if item.contains('<') {
                            let (name, val_map) =
                                item.split_once('<').expect("Already Checked delimiter");
                            let (val, map) =
                                val_map.split_once(':').expect("Already Checked delimiter");
                            let val = val.parse::<u64>().expect("Will parse");
                            (Some(name.into()), Some(Rule::LT), Some(val), map)
                        } else if item.contains('>') {
                            let (name, val_map) =
                                item.split_once('>').expect("Already Checked delimiter");
                            let (val, map) =
                                val_map.split_once(':').expect("Already Checked delimiter");
                            let val = val.parse::<u64>().expect("Will Parse");
                            (Some(name.into()), Some(Rule::GT), Some(val), map)
                        } else {
                            panic!("Found : unknown error.");
                        }
                    } else {
                        (None, None, None, item)
                    }
                })
                .collect();
            (key, vals)
        })
        .collect()
}

fn part_two(file: &str) -> u64 {
    let map = make_map(file);
    let map = Arc::new(Mutex::new(map));
    let mut ranges = Ranges::default();
    ranges.calculate(Arc::clone(&map), "in")
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
    const TEST: &str = "px{a<2006:qkq,m>2090:A,rfg}\n\
                        pv{a>1716:R,A}\n\
                        lnx{m>1548:A,A}\n\
                        rfg{s<537:gd,x>2440:R,A}\n\
                        qs{s>3448:A,lnx}\n\
                        qkq{x<1416:A,crn}\n\
                        crn{x>2662:A,R}()\n\
                        in{s<1351:px,qqz}\n\
                        qqz{s>2770:qs,m<1801:hdj,R}\n\
                        gd{a>3333:R,R}\n\
                        hdj{m>838:A,pv}\n\
                        \n\
                        {x=787,m=2655,a=1222,s=2876}\n\
                        {x=1679,m=44,a=2067,s=496}\n\
                        {x=2036,m=264,a=79,s=2244}\n\
                        {x=2461,m=1339,a=466,s=291}\n\
                        {x=2127,m=1623,a=2188,s=1013}";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 19114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST), 167409079868000);
    }
}

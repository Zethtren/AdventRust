use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn part_one(file: &str) -> u128 {
    file.to_string()
        .replace('\n', "")
        .par_split(',')
        .map(|hash| {
            hash.chars()
                .fold(0, |acc, c| (((acc + (c as u128)) * 17) % 256))
        })
        .sum()
}

fn part_two(file: &str) -> u128 {
    let mut boxes: HashMap<u128, Vec<(&str, u128)>> = HashMap::new();
    let items = file.to_string().replace('\n', "");
    items.split(',').for_each(|seq| {
        let op: char = if seq.contains('=') { '=' } else { '-' };
        let vals = seq.split(&['-', '=']).collect::<Vec<&str>>();
        let &box_name = vals.first().unwrap();
        let box_num = box_name
            .chars()
            .fold(0, |acc, c| (((acc + (c as u128)) * 17) % 256));
        if op == '=' {
            if let Some(i) = &boxes
                .entry(box_num)
                .or_insert(Vec::new().to_owned())
                .iter()
                .position(|&x| x.0 == box_name)
            {
                boxes.get_mut(&box_num).unwrap()[i.to_owned()] =
                    (box_name, vals.last().unwrap().parse::<u128>().unwrap()).to_owned();
            } else {
                boxes
                    .get_mut(&box_num)
                    .unwrap()
                    .push((box_name, vals.last().unwrap().parse::<u128>().unwrap()).to_owned());
            }
        } else if let Some(b) = boxes.get_mut(&box_num) {
            if let Some(old) = b.iter().position(|&a| a.0 == box_name) {
                b.remove(old);
            }
        }
    });
    boxes
        .into_iter()
        .map(|(b, vals)| {
            vals.iter()
                .enumerate()
                .map(|(lens, (_, strength))| (b + 1) * (lens as u128 + 1) * strength)
                .sum::<u128>()
        })
        .sum()
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
    static TST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 1320);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 145);
    }
}

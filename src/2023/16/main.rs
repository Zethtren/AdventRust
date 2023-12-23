use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    L,
    R,
    U,
    D,
}

type Grid = Arc<Mutex<HashMap<(i32, i32), (bool, char, Vec<Dir>)>>>;

fn plot_laser(direction: Dir, x: i32, y: i32, data: Grid) {
    let mut grid = data.lock().unwrap();
    if let Some((energized, c, dirs)) = grid.get_mut(&(x, y)) {
        if *energized & dirs.contains(&direction) {
            return;
        }
        *energized = true;
        dirs.push(direction);
        match (c, direction) {
            ('|', Dir::R) | ('|', Dir::L) => {
                drop(grid);
                plot_laser(Dir::U, x, y - 1, Arc::clone(&data));
                plot_laser(Dir::D, x, y + 1, Arc::clone(&data));
            }
            ('-', Dir::D) | ('-', Dir::U) => {
                drop(grid);
                plot_laser(Dir::L, x - 1, y, Arc::clone(&data));
                plot_laser(Dir::R, x + 1, y, Arc::clone(&data));
            }
            ('\\', Dir::U) | ('/', Dir::D) | ('-', Dir::L) | ('.', Dir::L) => {
                drop(grid);
                plot_laser(Dir::L, x - 1, y, Arc::clone(&data));
            }
            ('\\', Dir::D) | ('/', Dir::U) | ('-', Dir::R) | ('.', Dir::R) => {
                drop(grid);
                plot_laser(Dir::R, x + 1, y, Arc::clone(&data));
            }
            ('\\', Dir::R) | ('/', Dir::L) | ('|', Dir::D) | ('.', Dir::D) => {
                drop(grid);
                plot_laser(Dir::D, x, y + 1, Arc::clone(&data));
            }
            ('\\', Dir::L) | ('/', Dir::R) | ('|', Dir::U) | ('.', Dir::U) => {
                drop(grid);
                plot_laser(Dir::U, x, y - 1, Arc::clone(&data));
            }
            _ => {}
        }
    }
}

fn part_one(file: &str) -> u128 {
    let grid: Grid = Arc::new(Mutex::new(HashMap::new()));
    file.split('\n').enumerate().for_each(|(y, line)| {
        line.trim().chars().enumerate().for_each(|(x, t)| {
            grid.lock()
                .unwrap()
                .insert((x as i32, y as i32), (false, t, Vec::new()));
        })
    });
    plot_laser(Dir::R, 0, 0, Arc::clone(&grid));
    let fin = grid.lock().unwrap();
    fin.iter().filter(|point| point.1 .0).count() as u128
}

fn part_two(file: &str) -> u128 {
    let width = file.lines().next().unwrap().trim().len() as i32 - 1;
    let height = file.lines().count() as i32 - 1;

    let starts = vec![
        (0, 0, Dir::R),
        (0, 0, Dir::D),
        (width, 0, Dir::L),
        (width, 0, Dir::D),
        (width, height, Dir::L),
        (width, height, Dir::U),
        (0, height, Dir::U),
        (0, height, Dir::R),
    ];

    starts
        .into_par_iter()
        .map(|(x_s, y_s, dir)| {
            let grid: Grid = Arc::new(Mutex::new(HashMap::new()));
            file.split('\n').enumerate().for_each(|(y, line)| {
                line.trim().chars().enumerate().for_each(|(x, t)| {
                    grid.lock()
                        .unwrap()
                        .insert((x as i32, y as i32), (false, t, Vec::new()));
                })
            });
            plot_laser(dir, x_s, y_s, Arc::clone(&grid));
            let fin = grid.lock().unwrap();
            fin.iter().filter(|point| point.1 .0).count() as u128
        })
        .collect::<Vec<u128>>()
        .into_iter()
        .max()
        .unwrap()
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
    static TST: &str = ".|...\\....\n\
                        |.-.\\.....\n\
                        .....|-...\n\
                        ........|.\n\
                        ..........\n\
                        .........\\\n\
                        ..../.\\\\..\n\
                        .-.-/..|..\n\
                        .|....-|.\\\n\
                        ..//.|....";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 46);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 1);
    }
}


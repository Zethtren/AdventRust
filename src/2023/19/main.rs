use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn make_grid(file: &str) -> Vec<Vec<(bool, bool)>> {
    let mut pts: Vec<(i32, i32)> = Vec::new();
    file.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.trim().split(' ');
            if let (Some(dir), Some(dis)) = (parts.next(), parts.next()) {
                if let (Some(direction), Ok(distance)) = (dir.chars().next(), dis.parse::<u32>()) {
                    (direction, distance)
                } else {
                    panic!("Bad Parse");
                }
            } else {
                panic!("Bad parse on line");
            }
        })
        .for_each(|(dir, dist)| {
            match dir {
                'R' => (0..dist).for_each(|_| {
                    let cur = *pts.last().unwrap_or(&(0, 0));
                    pts.push((cur.0 + 1, cur.1));
                }),
                'L' => (0..dist).for_each(|_| {
                    let cur = *pts.last().unwrap_or(&(0, 0));
                    pts.push((cur.0 - 1, cur.1));
                }),
                'U' => (0..dist).for_each(|_| {
                    let cur = *pts.last().unwrap_or(&(0, 0));
                    pts.push((cur.0, cur.1 - 1));
                }),
                'D' => (0..dist).for_each(|_| {
                    let cur = *pts.last().unwrap_or(&(0, 0));
                    pts.push((cur.0, cur.1 + 1))
                }),
                _ => {}
            };
        });
    pts.sort_by(|(a, _), (b, _)| a.cmp(b));
    let min_x = pts.first().unwrap().0;
    let max_x = pts.last().unwrap().0 + 1;
    pts.sort_by(|(_, a), (_, b)| a.cmp(b));
    let min_y = pts.first().unwrap().1;
    let max_y = pts.last().unwrap().1 + 1;
    let mut grid =
        vec![vec![(false, false); (max_y - min_y) as usize]; (max_x - min_x) as usize];
    pts.iter().for_each(|pt| {
        grid[(pt.0 - min_x) as usize][(pt.1 - min_y) as usize] = (true, false)
    });
    grid
}

fn fill_grid(mut grid: Vec<Vec<(bool, bool)>> ) -> Vec<Vec<(bool, bool)>>  {
    let x = grid.len();
    if let Some(y) = grid.first().map(|row| row.len()) {
        (0..x).for_each(|ix| {
            (0..y).for_each(|iy| {
                // Left Border
                if grid[ix][iy].0 
                    || (((ix == 1) || (ix == 0)) && grid[0][iy].0)
                    // Top Border
                    || (((iy == 1) || (iy == 0)) && grid[ix][0].0)
                    // Centers
                    || ((ix > 1)
                        && (iy > 0)
                        && (grid[ix - 1][iy].0 && !grid[ix - 2][iy].1))
                    || ((ix > 0)
                        && (iy > 1)
                        && (grid[ix][iy - 1].0 && !grid[ix][iy - 2].1))
                    || ((ix > 0) && (!grid[ix -1][iy].0 && grid[ix - 1][iy].1))
                    || ((iy > 0) && (!grid[ix][iy - 1].0 && grid[ix][iy -1 ].1))
                {
                    if let Some((_, inner)) = grid.get_mut(ix).and_then(|row| row.get_mut(iy)) {
                        *inner = true;
                    }
                }
            })
        })
    }
    grid
}

fn print_border(grid: &[Vec<(bool, bool)>]) {
    grid.iter().for_each(|line| {
        println!();
        line.iter().for_each(|pt| {
            if pt.0 {
                print!("#");
            } else {
                print!(" ");
            }
        })
    });
}

fn print_filled(grid: &[Vec<(bool, bool)>]) {
    grid.iter().for_each(|line| {
        println!();
        line.iter().for_each(|pt| {
            if pt.1 {
                print!("#");
            } else {
                print!(" ");
            }
        })
    });
}

fn count_filled(grid: &[Vec<(bool, bool)>]) -> usize{
    grid.iter().flat_map(|line| {
        line.iter().filter(|pt|pt.1)
    }).count()
}

fn colors_to_directions(file: &str) -> String {
    file.lines().map(|line| {
        let items = line.trim().split(' ');
        if let Some(color) = items.last() {
            let mut col_str: String = color.chars().filter(|c| c.is_alphanumeric() ).collect();
            let dir = match col_str.pop() {
                Some('0') => 'R',
                Some('1') => 'D',
                Some('2') => 'L',
                Some('3') => 'U',
                _ => panic!("Bad Dir")
            };
            if let Ok(steps) = u32::from_str_radix(&col_str, 16) {
                format!("{} {}", dir, steps)
            } else {
                panic!("Bad Steps")
            }
        } else {
            panic!("Bad Color")
        }
    }).fold("".to_string(), |acc, line| [acc, line].join("\n"))
}


fn part_one(file: &str) -> usize {
    let grid = make_grid(file);
    let filled = fill_grid(grid);
    // print_border(&filled);
    // print_filled(&filled);
    count_filled(&filled)
}


fn part_two(file: &str) -> usize {
    let input = colors_to_directions(file);
    let grid = make_grid(&input);
    let filled = fill_grid(grid);
    count_filled(&filled)
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


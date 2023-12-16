use std::fs::read_to_string;
use std::num;
use std::{env, u128};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cosmos {
    Star,
    Space,
}

impl From<char> for Cosmos {
    fn from(value: char) -> Self {
        match value {
            '#' => Cosmos::Star,
            _ => Cosmos::Space,
        }
    }
}

impl From<Cosmos> for char {
    fn from(value: Cosmos) -> char {
        match value {
            Cosmos::Star => '#',
            Cosmos::Space => '.',
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Star {
    x: usize,
    y: usize,
}

impl Star {
    fn new(x: usize, y: usize) -> Self {
        Star { x, y }
    }
}

struct Game {
    _grid: Vec<Vec<Cosmos>>,
    _stars: Vec<Star>,
}

impl Game {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|cosmos| cosmos.into()).collect())
            .collect();
        Game {
            _grid: grid,
            _stars: Vec::new(),
        }
    }

    fn expand_universe(&mut self) {
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        for y in 0..self._grid.len() {
            if self._grid[y]
                .clone()
                .into_iter()
                .all(|x| x == Cosmos::Space)
            {
                rows.push(y);
            }
        }
        for x in 0..self._grid[0].len() {
            if self
                ._grid
                .clone()
                .into_iter()
                .map(|row| row[x])
                .all(|x| x == Cosmos::Space)
            {
                cols.push(x);
            }
        }
        let mut new_grid = self._grid.clone();
        for col in cols.into_iter().rev() {
            for row in 0..self._grid.len() {
                new_grid[row].insert(col, Cosmos::Space);
            }
        }
        for row in rows.into_iter().rev() {
            new_grid.insert(row, vec![Cosmos::Space; new_grid[0].len()])
        }
        self._grid = new_grid;
    }

    fn print_universe(&self) {
        for line in self._grid.clone() {
            let chars: Vec<char> = line.iter().map(|&x| x.into()).collect();
            for char in chars {
                print!("{}", char);
            }
            print!("\n");
        }
    }

    fn build_star_map(&mut self) {
        let mut stars = Vec::new();
        self._grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, &item)| {
                if item == Cosmos::Star {
                    stars.push(Star::new(x, y));
                }
            })
        });
        self._stars = stars;
    }

    fn pretend_to_spread_stars(&mut self) {
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        for y in 0..self._grid.len() {
            if self._grid[y]
                .clone()
                .into_iter()
                .all(|x| x == Cosmos::Space)
            {
                rows.push(y);
            }
        }
        for x in 0..self._grid[0].len() {
            if self
                ._grid
                .clone()
                .into_iter()
                .map(|row| row[x])
                .all(|x| x == Cosmos::Space)
            {
                cols.push(x);
            }
        }
        let mut new_stars = self._stars.clone();
        for row in rows.iter().rev() {
            let mut temp = Vec::new();
            for star in new_stars.into_iter() {
                if &star.y > row {
                    let star = Star::new(star.x, (star.y + 999_999));
                    temp.push(star);
                } else {
                    temp.push(star);
                }
            }
            new_stars = temp;
        }
        for col in cols.iter().rev() {
            let mut temp = Vec::new();
            for star in new_stars.into_iter() {
                if &star.x > col {
                    let star = Star::new(star.x + 999_999, star.y);
                    temp.push(star);
                } else {
                    temp.push(star);
                }
            }
            new_stars = temp;
        }
        for star in &new_stars {
            println!("{:?}", star);
        }
        self._stars = new_stars;
    }

    fn calculate_distane(&self) -> u128 {
        self._stars[..self._stars.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, star)| {
                self._stars[i + 1..].iter().fold(0, |acc, to| {
                    acc + ((to.x as i64 - star.x as i64).abs()
                        + (to.y as i64 - star.y as i64).abs()) as u128
                })
            })
            .sum()
    }

    fn part_one(&mut self) -> u128 {
        self.expand_universe();
        self.build_star_map();
        self.calculate_distane()
    }

    fn part_two(&mut self) -> u128 {
        self.build_star_map();
        self.pretend_to_spread_stars();
        self.calculate_distane()
    }
}

fn part_one(input: &mut Game) -> u128 {
    input.part_one()
}

fn part_two(input: &mut Game) -> u128 {
    input.part_two()
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let file = read_to_string(args.remove(0)).unwrap();
    let mut game = Game::new(&file);
    let res1 = part_one(&mut game);
    println!("Part 1: {}", res1);
    let mut game = Game::new(&file);
    let res2 = part_two(&mut game);
    println!("Part 2: {}", res2);
}

#[cfg(test)]
mod test {
    static TEST: &str = "...#......\n\
                        .......#..\n\
                        #.........\n\
                        ..........\n\
                        ......#...\n\
                        .#........\n\
                        .........#\n\
                        ..........\n\
                        .......#..\n\
                        #...#.....";

    use super::{part_one, part_two, Game};

    // #[test]
    // fn test_expand() {
    //     let game = Game::new(TEST);
    //     game.expand_universe();
    //     // assert_eq!(game._grid, TEST_ANSWER)
    // }

    #[test]
    fn test_part_one() {
        let mut game = Game::new(TEST);
        assert_eq!(part_one(&mut game), 374);
    }
}

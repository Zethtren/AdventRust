use std::{env, fs::read_to_string};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PointType {
    Unknown,
    Inner,
    Outer,
    Border,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum MapType {
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
    None,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct MapPoint {
    m: MapType,
    p: PointType,
}

impl MapPoint {
    fn update_type(&mut self, t: PointType) {
        self.p = t;
    }
}

impl From<char> for MapPoint {
    fn from(value: char) -> MapPoint {
        let pt = PointType::Unknown;
        match value {
            'S' => MapPoint {
                m: MapType::Start,
                p: pt,
            },
            '.' => MapPoint {
                m: MapType::Ground,
                p: pt,
            },
            '|' => MapPoint {
                m: MapType::Vertical,
                p: pt,
            },
            '-' => MapPoint {
                m: MapType::Horizontal,
                p: pt,
            },
            '7' => MapPoint {
                m: MapType::SouthWest,
                p: pt,
            },
            'F' => MapPoint {
                m: MapType::SouthEast,
                p: pt,
            },
            'L' => MapPoint {
                m: MapType::NorthEast,
                p: pt,
            },
            'J' => MapPoint {
                m: MapType::NorthWest,
                p: pt,
            },
            _ => MapPoint {
                m: MapType::None,
                p: pt,
            },
        }
    }
}

#[derive(Debug)]
enum PreviousMove {
    Up,
    Down,
    Right,
    Left,
    None,
}

#[derive(Debug)]
struct Grid {
    _grid: Vec<Vec<MapPoint>>,
    _prev_move: PreviousMove,
    _move_count: u32,
    _current_state: (usize, usize),
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<MapPoint>> = input
            .lines()
            .map(|line| line.chars().map(|x| x.into()).collect())
            .collect();
        let mut start = (0, 0);
        for (i, row) in grid.iter().enumerate() {
            if let Some(j) = row.iter().position(|x| x.m == MapType::Start) {
                start = (j, i)
            } else {
                continue;
            }
        }
        Grid {
            _grid: grid,
            _prev_move: PreviousMove::None,
            _move_count: 0,
            _current_state: start,
        }
    }

    fn get_point(&mut self, x: usize, y: usize) -> &mut MapPoint {
        &mut self._grid[y][x]
    }

    fn move_left(&mut self) {
        self._prev_move = PreviousMove::Left;
        self._current_state = (self._current_state.0 - 1, self._current_state.1);
    }

    fn move_right(&mut self) {
        self._prev_move = PreviousMove::Right;
        self._current_state = (self._current_state.0 + 1, self._current_state.1);
    }

    fn move_up(&mut self) {
        self._prev_move = PreviousMove::Up;
        self._current_state = (self._current_state.0, self._current_state.1 - 1);
    }

    fn move_down(&mut self) {
        self._prev_move = PreviousMove::Down;
        self._current_state = (self._current_state.0, self._current_state.1 + 1);
    }

    fn move_start(&mut self) {
        let start = self.get_point(self._current_state.0, self._current_state.1);
        start.update_type(PointType::Border);
        self._move_count += 1;
        if self._current_state.0 > 0 {
            let next = (self._current_state.0 - 1, self._current_state.1);
            match self.get_point(next.0, next.1).m {
                MapType::SouthEast | MapType::NorthEast | MapType::Horizontal => {
                    self.move_left();
                    return;
                }
                _ => {}
            }
        }
        if self._current_state.0 < (self._grid.len() - 1) {
            let next = (self._current_state.0 + 1, self._current_state.1);
            match self.get_point(next.0, next.1).m {
                MapType::SouthWest | MapType::NorthWest | MapType::Horizontal => {
                    self.move_right();
                    return;
                }
                _ => {}
            }
        }
        if self._current_state.1 > 0 {
            let next = (self._current_state.0, self._current_state.1 - 1);
            match self.get_point(next.0, next.1).m {
                MapType::NorthWest | MapType::NorthEast | MapType::Vertical => {
                    self.move_down();
                    return;
                }
                _ => {}
            }
        }
        if self._current_state.1 < (self._grid[0].len() - 1) {
            let next = (self._current_state.0, self._current_state.1 + 1);
            match self.get_point(next.0, next.1).m {
                MapType::SouthEast | MapType::SouthWest | MapType::Vertical => {
                    self.move_up();
                }
                _ => {}
            }
        }
    }

    fn move_fwd_keep_going(&mut self) -> bool {
        let (x, y) = self._current_state;
        self._move_count += 1;
        let current_point = self.get_point(x, y);

        match current_point.m {
            MapType::NorthWest => {
                current_point.update_type(PointType::Border);
                match self._prev_move {
                    PreviousMove::Down => {
                        self.move_left();
                    }
                    PreviousMove::Right => {
                        self.move_up();
                    }
                    _ => {
                        panic!("Unexpected Error")
                    }
                }
            }
            MapType::NorthEast => {
                current_point.update_type(PointType::Border);
                match self._prev_move {
                    PreviousMove::Down => {
                        self.move_right();
                    }
                    PreviousMove::Left => {
                        self.move_up();
                    }
                    _ => {
                        panic!("Unexpected Error")
                    }
                }
            }
            MapType::SouthEast => {
                current_point.update_type(PointType::Border);
                match self._prev_move {
                    PreviousMove::Up => {
                        self.move_right();
                    }
                    PreviousMove::Left => {
                        self.move_down();
                    }
                    _ => {
                        panic!("Unexpected Error")
                    }
                }
            }
            MapType::SouthWest => {
                current_point.update_type(PointType::Border);
                match self._prev_move {
                    PreviousMove::Up => {
                        self.move_left();
                    }
                    PreviousMove::Right => {
                        self.move_down();
                    }
                    _ => {
                        panic!("Unexpected Error")
                    }
                }
            }
            MapType::Vertical => {
                current_point.update_type(PointType::Border);
                match self._prev_move {
                    PreviousMove::Up => {
                        self.move_up();
                    }
                    PreviousMove::Down => {
                        self.move_down();
                    }
                    _ => {
                        panic!("Unexpected Error")
                    }
                }
            }
            MapType::Horizontal => {
                current_point.update_type(PointType::Border);
                match self._prev_move {
                    PreviousMove::Left => {
                        self.move_left();
                    }
                    PreviousMove::Right => {
                        self.move_right();
                    }
                    _ => {
                        panic!("Unexpected Error")
                    }
                }
            }
            MapType::Start => {
                return false;
            }
            _ => {
                panic!("Unexpected Error on points")
            }
        }
        true
    }
    fn play_one(&mut self) -> u32 {
        self.move_start();
        while self.move_fwd_keep_going() {}
        self._move_count / 2
    }
    fn play_two(&mut self) -> u32 {
        let mut inner = 0;
        self._grid.clone().into_iter().for_each(|row| {
            let mut acc = 0;
            let mut north = false;
            row.into_iter().for_each(|mut point| {
                if point.p == PointType::Border {
                    match point.m {
                        MapType::Vertical => acc += 1,
                        MapType::NorthEast => north = true,
                        MapType::SouthEast => north = false,
                        MapType::NorthWest => {
                            if north {
                                acc += 2;
                            } else {
                                acc += 1;
                            }
                        }
                        MapType::SouthWest => {
                            if north {
                                acc += 1;
                            } else {
                                acc += 2;
                            }
                        }
                        _ => {}
                    };
                } else if acc % 2 == 0 {
                    point.update_type(PointType::Outer);
                } else {
                    point.update_type(PointType::Inner);
                    inner += 1;
                }
            })
        });
        inner
    }
}

fn part_one(input: &mut Grid) -> u32 {
    input.play_one()
}

fn part_two(input: &mut Grid) -> u32 {
    input.play_two()
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let file = read_to_string(args.remove(0)).unwrap();
    let mut grid = Grid::new(&file);
    let res1 = part_one(&mut grid);
    println!("Part 1: {}", res1);
    let res2 = part_two(&mut grid);
    println!("Part 2: {}", res2);
}

#[cfg(test)]
mod test {
    static TEST: &str = "..F7.\n\
                        .FJ|.\n\
                        SJ.L7\n\
                        |F--J\n\
                        LJ...";

    use super::{part_one, part_two, Grid};

    #[test]
    fn test_part_one() {
        let mut grid = Grid::new(TEST);
        assert_eq!(part_one(&mut grid), 8);
    }

    #[test]
    fn test_part_two() {
        let mut grid = Grid::new(TEST);
        assert_eq!(part_two(&mut grid), 2);
    }
}

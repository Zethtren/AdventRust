use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (i32, i32),
    diff: (i32, i32),
    n: u8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    cost_grid: &[Vec<u32>],
    start: (i32, i32),
    goal: (i32, i32),
    ultra_crucible: bool,
) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut set = HashSet::new();

    heap.push(State {
        cost: 0,
        position: start,
        diff: start,
        n: 0,
    });

    while let Some(State {
        cost,
        position,
        diff,
        n,
    }) = heap.pop()
    {
        if position == goal {
            return Some(cost);
        }

        if set.contains(&(position, diff, n)) {
            continue;
        }
        set.insert((position, diff, n));

        let cap = if ultra_crucible { 10 } else { 3 };

        if (n < cap) && (diff != (0, 0)) {
            let (nx, ny) = ((position.0 + diff.0), (position.1 + diff.1));
            if (nx >= 0) && (nx <= goal.0) && (ny >= 0) && (ny <= goal.1) {
                heap.push(State {
                    cost: cost + cost_grid[ny as usize][nx as usize],
                    position: (nx, ny),
                    diff,
                    n: n + 1,
                });
            }
        }

        if !ultra_crucible || ((n >= 4) || diff == (0, 0)) {
            for (ndx, ndy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if ((ndx, ndy) != (diff.0, diff.1)) && ((ndx, ndy) != (-diff.0, -diff.1)) {
                    let (nx, ny) = ((position.0 + ndx), (position.1 + ndy));
                    if (nx >= 0) && (nx <= goal.0) && (ny >= 0) && (ny <= goal.1) {
                        heap.push(State {
                            cost: cost + cost_grid[ny as usize][nx as usize],
                            position: (nx, ny),
                            diff: (ndx, ndy),
                            n: 1,
                        });
                    }
                }
            }
        }
    }
    // Goal not reachable
    None
}

fn part_one(file: &str) -> u32 {
    if let Some(grid) = file
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|num| num.to_digit(10))
                .collect::<Option<Vec<u32>>>()
        })
        .collect::<Option<Vec<Vec<u32>>>>()
    {
        let trimmed_grid = grid
            .into_iter()
            .filter(|line| line.len() > 0)
            .collect::<Vec<Vec<u32>>>();

        if let Some(path) = shortest_path(
            &trimmed_grid,
            (0, 0),
            (
                trimmed_grid[0].len() as i32 - 1,
                trimmed_grid.len() as i32 - 1,
            ),
            false,
        ) {
            return path;
        }
    }
    0
}

fn part_two(file: &str) -> u32 {
    if let Some(grid) = file
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|num| num.to_digit(10))
                .collect::<Option<Vec<u32>>>()
        })
        .collect::<Option<Vec<Vec<u32>>>>()
    {
        let trimmed_grid = grid
            .into_iter()
            .filter(|line| line.len() > 0)
            .collect::<Vec<Vec<u32>>>();

        if let Some(path) = shortest_path(
            &trimmed_grid,
            (0, 0),
            (
                trimmed_grid[0].len() as i32 - 1,
                trimmed_grid.len() as i32 - 1,
            ),
            true,
        ) {
            return path;
        }
    }
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
    static TST: &str = "2413432311323\n\
                        3215453535623\n\
                        3255245654254\n\
                        3446585845452\n\
                        4546657867536\n\
                        1438598798454\n\
                        4457876987766\n\
                        3637877979653\n\
                        4654967986887\n\
                        4564679986453\n\
                        1224686865563\n\
                        2546548887735\n\
                        4322674655533";

    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TST), 102);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TST), 1);
    }
}


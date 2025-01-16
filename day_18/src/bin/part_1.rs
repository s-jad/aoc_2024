use itertools::Itertools;
use std::{collections::VecDeque, time::Instant};

const GRID_DIMS: usize = 71;
const GOAL_COORDS: usize = 70;
const BYTES: usize = 1024;

const GRID_X: usize = GRID_DIMS;
const GRID_Y: usize = GRID_DIMS;
const GOAL: (usize, usize) = (GOAL_COORDS, GOAL_COORDS);

#[derive(Debug)]
struct MemorySpace {
    width: i32,
    height: i32,
    grid: [[bool; GRID_X]; GRID_Y],
}

impl MemorySpace {
    fn new(width: i32, height: i32) -> Self {
        MemorySpace {
            width,
            height,
            grid: [[true; GRID_X]; GRID_Y],
        }
    }

    fn is_valid(&self, x: i32, y: i32, ix: usize, iy: usize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height && self.grid[iy][ix]
    }

    fn shortest_path(&self) -> usize {
        let mut dp = vec![vec![-1i32; GRID_X]; GRID_Y];
        let mut queue = VecDeque::new();
        let start = (0i32, 0i32);
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        dp[0][0] = 0;
        queue.push_back(start);

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in dirs {
                let nx = x + dx;
                let ny = y + dy;
                let nix = nx as usize;
                let niy = ny as usize;
                let ix = x as usize;
                let iy = y as usize;

                if self.is_valid(nx, ny, ix, iy) && dp[niy][nix] == -1 {
                    dp[niy][nix] = dp[iy][ix] + 1;
                    queue.push_back((nx, ny));
                }
            }
        }

        dp[GOAL.1][GOAL.0].max(0) as usize
    }
}

fn process(input: &str) -> usize {
    let mut space = MemorySpace::new(GRID_X as i32, GRID_Y as i32);

    for (col, row) in input.lines().take(BYTES).map(|l| {
        l.split_terminator(&[',', ' ', '\n'])
            .map(|s| s.parse::<usize>().expect("should parse"))
            .collect_tuple::<(_, _)>()
            .expect("two numbers")
    }) {
        space.grid[row][col] = false;
    }

    space.shortest_path()
}
fn debug_grid(grid: &[[bool; GRID_X]; GRID_Y]) {
    println!("--------- GRID ------------");
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{:>2}", if *cell { "." } else { "#" });
        }
        println!();
    }
    println!("---------------------------");
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}

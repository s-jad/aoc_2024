use itertools::Itertools;
use std::time::Instant;

#[derive(Debug)]
struct Grid {
    robot: (usize, usize),
    walls: Vec<(usize, usize)>,
    max_row: usize,
    max_col: usize,
}

impl Grid {
    fn new() -> Self {
        Self {
            robot: (0, 0),
            walls: vec![],
            max_row: 0,
            max_col: 0,
        }
    }
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn scan(ins: &Dir, x: usize, y: usize) -> (usize, usize) {
    match ins {
        Dir::Left => (x, y - 1),
        Dir::Right => (x, y + 1),
        Dir::Up => (x - 1, y),
        Dir::Down => (x + 1, y),
    }
}

fn get_board(p1: &str, p2: &str) -> (Grid, Vec<Dir>, Vec<(usize, usize)>) {
    let mut boxes = vec![];

    let grid = p1
        .lines()
        .enumerate()
        .fold(Grid::new(), |mut grid, (row, l)| {
            let v = l.trim().chars().enumerate().collect_vec();

            for (col, c) in v {
                match c {
                    '@' => grid.robot = (row, col),
                    'O' => boxes.push((row, col)),
                    '#' => grid.walls.push((row, col)),
                    _ => {}
                }

                grid.max_col = col.max(grid.max_col);
            }

            grid.max_row = row.max(grid.max_row);

            grid
        });

    let ins = p2
        .lines()
        .flat_map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    '<' => Dir::Left,
                    '>' => Dir::Right,
                    '^' => Dir::Up,
                    'v' => Dir::Down,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    debug_grid(&grid, &boxes);
    (grid, ins, boxes)
}

fn debug_grid(grid: &Grid, boxes: &[(usize, usize)]) {
    println!("\n------------- GRID --------------\n");
    for row in 0..=grid.max_row {
        for col in 0..=grid.max_col {
            if grid.walls.contains(&(row, col)) {
                print!("#");
            } else if boxes.contains(&(row, col)) {
                print!("O");
            } else if grid.robot == (row, col) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn search_path(
    grid: &Grid,
    boxes: &mut Vec<(usize, usize)>,
    dir: &Dir,
    (x, y): (usize, usize),
) -> Option<(usize, usize)> {
    let next = scan(dir, x, y);

    if grid.walls.contains(&next) {
        return None;
    }

    if let Some(bidx) = boxes.iter().position(|coord| *coord == next) {
        if let Some(new_coord) = search_path(grid, boxes, dir, next) {
            boxes[bidx] = new_coord;
            return Some(next);
        } else {
            return None;
        }
    }

    Some(next)
}

fn process(input: &str) -> usize {
    let (p1, p2) = input
        .split("\n\n")
        .collect_tuple::<(_, _)>()
        .expect("two parts");

    let (mut grid, ins, mut boxes) = get_board(p1, p2);

    for i in ins.iter() {
        if let Some(rcoord) = search_path(&grid, &mut boxes, i, grid.robot) {
            grid.robot = rcoord;
        }
    }

    boxes.into_iter().map(|b| (b.0 * 100) + b.1).sum()
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

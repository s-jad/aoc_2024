use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn check_routes(
    grid: &[Vec<u8>],
    (max_row, max_col): (i32, i32),
    (sr, sc): (i32, i32),
    dirs: [(i32, i32); 4],
    prev: u8,
    (prev_r, prev_c): (i32, i32),
) -> usize {
    let mut total = 0;

    for (dr, dc) in dirs
        .iter()
        .filter(|&&(dr, dc)| (dr, dc) != (-prev_r, -prev_c))
    {
        let (cr, cc) = (sr + dr, sc + dc);
        if cr >= 0 && cc >= 0 && cr < max_row && cc < max_col {
            let current = grid[cr as usize][cc as usize];
            if current == 9 && prev == 8 {
                total += 1;
            } else if (current as i16 - prev as i16) == 1 {
                total += check_routes(
                    grid,
                    (max_row, max_col),
                    (cr, cc),
                    dirs,
                    current,
                    (*dr, *dc),
                );
            }
        }
    }

    total
}

fn process(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| {
            l.trim_end()
                .chars()
                .map(|c| (c as u8).overflowing_sub(48).0)
                .collect_vec()
        })
        .collect_vec();

    let max_row = grid.len() as i32;
    let max_col = grid[0].len() as i32;
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    grid.iter()
        .enumerate()
        .flat_map(|(row, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, val)| **val == 0u8)
                .map(|(col, _)| (row as i32, col as i32))
                .collect_vec()
        })
        .fold(0, |acc, th| {
            acc + check_routes(&grid, (max_row, max_col), th, dirs, 0u8, (10, 10))
        })
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

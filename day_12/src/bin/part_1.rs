use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

const TEST_DIMS: usize = 4;
const TEST_DIMS2: usize = 10;
const TEST_NESTED: usize = 5;
const DIMS: usize = 140;

fn find_group(
    idx: usize,
    grid: &[char; DIMS * DIMS],
    visited: &mut HashSet<usize>,
) -> (usize, usize) {
    if !visited.insert(idx) {
        return (0, 0);
    }

    let mut m = 1;
    let mut b = 4;
    let mut pb = b;

    for n in [
        idx.wrapping_sub(1),
        idx + 1,
        idx.wrapping_sub(DIMS),
        idx + DIMS,
    ]
    .into_iter()
    {
        if n < (DIMS * DIMS) && grid[n] == grid[idx] && (n % DIMS).abs_diff(idx % DIMS) != DIMS - 1
        {
            b -= 1;
            pb -= 1;
            let (ob, om) = find_group(n, grid, visited);
            b += ob;
            m += om;
        }
    }

    (b, m)
}

fn get_group_scores(grid: &[char; DIMS * DIMS]) -> usize {
    let mut visited = HashSet::new();
    let glen = grid.len();
    let mut score = 0;

    while visited.len() != glen {
        for i in 0..DIMS * DIMS {
            if !visited.contains(&i) {
                let (b, m) = find_group(i, grid, &mut visited);
                //println!("group: {:?}", grid[i]);
                //println!("boundaries: {:?}", b);
                //println!("members: {:?}", m);
                score += m * b;
            }
        }
    }
    score
}

fn process(input: &str) -> usize {
    let grid: [char; DIMS * DIMS] = input
        .lines()
        .flat_map(|l| l.chars().collect_vec())
        .collect_array()
        .expect("should have DIMS*DIMS chars");

    //println!("grid: {:?}", grid);

    get_group_scores(&grid)
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

use itertools::Itertools;
use std::time::Instant;

#[derive(Debug)]
struct Grid {
    rows: i32,
    cols: i32,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(data: &str) -> Self {
        let mut rows = 0;

        let grid = data
            .lines()
            .map(|l| {
                let c = l.chars().collect_vec();
                rows += 1;
                c
            })
            .collect_vec();

        Self {
            rows,
            cols: grid[0].len() as i32,
            grid,
        }
    }

    fn find_occurances(&self) -> usize {
        let mut occurances = 0;
        let directions = [(1, -1), (1, 1), (-1, -1), (-1, 1)];

        for r in 1..(self.rows - 1) as usize {
            for c in 1..(self.cols - 1) as usize {
                if self.grid[r][c] == 'A' {
                    occurances += self.search_mas(r as i32, c as i32, &directions);
                }
            }
        }

        occurances
    }

    fn search_mas(&self, sr: i32, sc: i32, dir: &[(i32, i32); 4]) -> usize {
        let tl = self.grid[(sr + dir[0].0) as usize][(sc + dir[0].1) as usize];
        let tr = self.grid[(sr + dir[1].0) as usize][(sc + dir[1].1) as usize];
        let bl = self.grid[(sr + dir[2].0) as usize][(sc + dir[2].1) as usize];
        let br = self.grid[(sr + dir[3].0) as usize][(sc + dir[3].1) as usize];

        match (tl, tr, bl, br) {
            ('S', 'S', 'M', 'M')
            | ('M', 'M', 'S', 'S')
            | ('S', 'M', 'S', 'M')
            | ('M', 'S', 'M', 'S') => 1,
            _ => 0,
        }
    }
}

fn process(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.find_occurances()
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

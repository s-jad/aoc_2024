use itertools::Itertools
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

    fn find_occurances(&self, word: &str) -> usize {
        let mut occurances = 0;
        let directions = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];

        for r in 0..self.rows as usize {
            for c in 0..self.cols as usize {
                if self.grid[r][c] == 'X' {
                    occurances += self.search_word(word, r as i32, c as i32, &directions);
                    println!("checked: ({:?}, {:?})\ntotal: {:?}", r, c, occurances)
                }
            }
        }

        occurances
    }

    fn search_word(
        &self,
        word: &str,
        start_row: i32,
        start_col: i32,
        dir: &[(i32, i32); 8],
    ) -> usize {
        let mut occurances = 0;

        for (dr, dc) in dir {
            let mut cr = start_row;
            let mut cc = start_col;

            for ch in word.chars() {
                if ch == self.grid[cr as usize][cc as usize] && ch == 'S' {
                    occurances += 1;
                    break;
                } else if ch != self.grid[cr as usize][cc as usize]
                    || cr + dr < 0
                    || cr + dr >= self.rows
                    || cc + dc < 0
                    || cc + dc >= self.cols
                {
                    break;
                } else {
                    cr += dr;
                    cc += dc;
                }
            }
        }

        occurances
    }
}

fn process(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.find_occurances("XMAS")
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

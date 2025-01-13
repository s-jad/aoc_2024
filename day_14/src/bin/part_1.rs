use itertools::Itertools;
use std::time::Instant;

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn new(pr: i32, pc: i32, vr: i32, vc: i32) -> Self {
        Self {
            pos: (pr, pc),
            vel: (vr, vc),
        }
    }

    fn get_quadrant(
        &self,
        max_row: i32,
        max_col: i32,
        mid_row: i32,
        mid_col: i32,
        seconds: i32,
    ) -> usize {
        let new_r = self.pos.0 + (self.vel.0 * seconds);
        let new_c = self.pos.1 + (self.vel.1 * seconds);

        let final_r = if new_r < 0 {
            (max_row + (new_r % max_row)) % max_row
        } else {
            new_r % max_row
        };

        let final_c = if new_c < 0 {
            (max_col + (new_c % max_col)) % max_col
        } else {
            new_c % max_col
        };

        match (final_r.cmp(&mid_row), final_c.cmp(&mid_col)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 0,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 2,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 3,
            _ => 4,
        }
    }
}

fn process(input: &str) -> usize {
    const MAX_COL: i32 = 101;
    const MID_COL: i32 = (MAX_COL - 1) / 2;

    const MAX_ROW: i32 = 103;
    const MID_ROW: i32 = (MAX_ROW - 1) / 2;

    const SECONDS: i32 = 100;

    let mut quad_counts = [0; 5];

    let robots = input
        .lines()
        .map(|l| {
            let (pc, pr, vc, vr) = l
                .split_terminator(&['=', ',', ' ', '\n', 'p', 'v'])
                .filter_map(|p| p.parse::<i32>().ok())
                .collect_tuple::<(_, _, _, _)>()
                .expect("Should be 4 values");

            Robot::new(pr, pc, vr, vc)
        })
        .collect_vec();

    for r in robots {
        let q = r.get_quadrant(MAX_ROW, MAX_COL, MID_ROW, MID_COL, SECONDS);

        quad_counts[q] += 1;
    }

    quad_counts[..4].iter().product()
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

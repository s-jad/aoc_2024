use itertools::Itertools;
use std::time::Instant;

fn check_solvability(current: usize, goal: &usize, nums: &[usize], mut idx: usize) -> bool {
    let mut solvable = false;

    if &current > goal {
        return false;
    }

    match (&current == goal, idx == nums.len()) {
        (true, true) | (true, false) => {
            return true;
        }
        (false, true) => {
            return false;
        }
        (false, false) => {
            let added = current + nums[idx];
            let multiplied = current * nums[idx];
            let concatenated = format!("{current}{}", nums[idx])
                .parse::<usize>()
                .expect("is number");

            idx += 1;

            if check_solvability(added, goal, nums, idx)
                || check_solvability(multiplied, goal, nums, idx)
                || check_solvability(concatenated, goal, nums, idx)
            {
                solvable = true;
            }
        }
    }

    solvable
}

fn process(input: &str) -> usize {
    let equations = input
        .lines()
        .map(|l| {
            l.split_terminator([':', '\n', ' '])
                .filter_map(|p| p.parse::<usize>().ok())
                .collect_vec()
        })
        .collect_vec();

    let mut calibrated = 0;

    for eq in equations {
        let current = eq[1];
        let goal = eq[0];
        if check_solvability(current, &goal, &eq[2..], 0) {
            calibrated += goal;
        }
    }

    calibrated
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

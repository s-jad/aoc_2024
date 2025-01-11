use itertools::Itertools;
use std::time::Instant;

fn str_to_num(input: &str) -> isize {
    input.chars().rev().enumerate().fold(0isize, |num, (i, c)| {
        let d = c.to_digit(10).unwrap_or(0) as isize;
        num + d * 10_u32.pow(i as u32) as isize
    })
}

fn process(input: &str) -> usize {
    let (mut l1, mut l2) = input.split_whitespace().enumerate().fold(
        (Vec::new(), Vec::new()),
        |(mut l1, mut l2), (i, num)| {
            if i % 2 == 0 {
                l1.push(str_to_num(num));
            } else {
                l2.push(str_to_num(num));
            }
            (l1, l2)
        },
    );

    l1.sort_unstable();
    l2.sort_unstable();

    l1.into_iter()
        .zip(l2)
        .map(|(a, b)| (b - a).abs())
        .sum::<isize>() as usize
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

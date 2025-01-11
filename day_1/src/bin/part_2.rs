use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn str_to_num(input: &str) -> i32 {
    input.chars().rev().enumerate().fold(0i32, |num, (i, c)| {
        let d = c.to_digit(10).unwrap_or(0) as i32;
        num + d * 10_u32.pow(i as u32) as i32
    })
}

fn process(input: &str) -> usize {
    let (mut v, key) = input.split_whitespace().enumerate().fold(
        (Vec::new(), HashMap::new()),
        |(mut v, mut k), (i, num)| {
            if i % 2 == 0 {
                v.push(str_to_num(num));
            } else {
                let n = str_to_num(num);
                *k.entry(n).or_insert(0) += 1;
            }
            (v, k)
        },
    );

    v.sort_unstable();

    v.into_iter()
        .map(|n| n * *key.get(&n).unwrap_or(&0))
        .sum::<i32>() as usize
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
    fn test_2() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}

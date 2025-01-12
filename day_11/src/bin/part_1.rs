//1. If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
//
//2. If the stone is engraved with a number that has an even number of digits, it is replaced by two
//stones. The left half of the digits are engraved on the new left stone, and the right half of the
//digits are engraved on the new right stone.
//(The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
//
//3. If none of the other rules apply, the stone is replaced by a new stone; the old stone's number
//multiplied by 2024 is engraved on the new stone.

use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn blink(val: &usize, val_str_len: usize) -> Option<usize> {
    if *val == 0 {
        Some(1)
    } else if val_str_len % 2 == 0 {
        None
    } else {
        Some(*val * 2024)
    }
}

fn process(input: &str) -> usize {
    let mut initial = input
        .split_terminator(&['\n', ' '])
        .filter_map(|p| p.parse::<usize>().ok())
        .enumerate()
        .fold(HashMap::new(), |mut acc: HashMap<usize, usize>, (i, n)| {
            acc.insert(i, n);
            acc
        });

    let mut next = initial.len();

    for _ in 0..25 {
        let mut new_vals = vec![];

        for (_, val) in initial.iter_mut() {
            let val_str = val.to_string();
            let val_str_len = val_str.len();
            if let Some(new_val) = blink(val, val_str_len) {
                *val = new_val;
            } else {
                let (p1, p2) = val_str.split_at(val_str_len / 2);
                *val = p1.parse::<usize>().expect("should be parseable value");
                new_vals.push((next, p2.parse::<usize>().ok()));
                next += 1;
            }
        }

        new_vals.into_iter().for_each(|(next, maybe_val)| {
            if let Some(v) = maybe_val {
                initial.insert(next, v);
            }
        });
    }

    initial.len()
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

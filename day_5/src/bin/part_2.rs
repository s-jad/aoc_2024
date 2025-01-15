use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn filter_correct_mids(ins: &HashMap<u8, Vec<u8>>, update: &mut [u8]) -> Option<usize> {
    let len = update.len();
    let mid = len / 2;
    let mut push = false;

    for _ in 0..mid {
        let mut changed = false;
        for cidx in (1..len).rev() {
            if let Some(denied) = ins.get(&update[cidx]) {
                for nidx in (0..cidx).rev() {
                    if denied.contains(&update[nidx]) {
                        update.swap(cidx, nidx);
                        push = true;
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    if push {
        Some(update[mid] as usize)
    } else {
        None
    }
}

fn process(input: &str) -> usize {
    let (p1, p2) = input
        .split_once("\n\n")
        .expect("Should find double newline");

    let ins = p1
        .lines()
        .fold(HashMap::new(), |mut map: HashMap<u8, Vec<u8>>, p| {
            let (k, v) = p
                .split("|")
                .map(|l| {
                    l.trim_end_matches("\n")
                        .parse::<u8>()
                        .expect("Should be digit!")
                })
                .collect_tuple::<(_, _)>()
                .expect("should be tuple");

            if let Some(vals) = map.get_mut(&k) {
                vals.push(v);
            } else {
                map.insert(k, vec![v]);
            }
            map
        });

    p2.lines()
        .map(|l| {
            l.trim_end_matches("\n")
                .split(",")
                .map(|l| l.parse::<u8>().expect("Should be digit!"))
                .collect_vec()
        })
        .filter_map(|mut update| filter_correct_mids(&ins, &mut update))
        .sum()
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

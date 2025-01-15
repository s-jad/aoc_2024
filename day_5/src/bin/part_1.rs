use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn filter_correct_updates(ins: HashMap<u8, Vec<u8>>, updates: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut allowed = vec![];

    'update_loop: for update in updates {
        let len = update.len();

        for cidx in (1..len).rev() {
            if let Some(denied) = ins.get(&update[cidx]) {
                for nidx in (0..cidx).rev() {
                    if denied.contains(&update[nidx]) {
                        println!(
                            "denied list '{}': {:?}\ncontains {:?}\n",
                            cidx, denied, update[nidx]
                        );
                        continue 'update_loop;
                    }
                }
            }
        }

        allowed.push(update);
    }

    allowed
}

fn calc_mid_vals(updates: Vec<Vec<u8>>) -> usize {
    let mut acc = 0;

    for update in updates {
        let mid = update.len() / 2;
        acc += update[mid] as usize;
    }

    acc
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

    let updates = p2
        .lines()
        .map(|l| {
            l.trim_end_matches("\n")
                .split(",")
                .map(|l| l.parse::<u8>().expect("Should be digit!"))
                .collect_vec()
        })
        .collect_vec();

    let correct = filter_correct_updates(ins, updates);

    calc_mid_vals(correct)
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

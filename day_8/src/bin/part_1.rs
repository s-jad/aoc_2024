use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

type PossibleAntinodes = (Option<(usize, usize)>, Option<(usize, usize)>);

fn find_antinodes(
    c1: &(usize, usize),
    c2: &(usize, usize),
    max_row: usize,
    max_col: usize,
) -> PossibleAntinodes {
    let (c1r, c1c, c2r, c2c) = (c1.0 as isize, c1.1 as isize, c2.0 as isize, c2.1 as isize);
    let dir1 = (c1r - c2r, c1c - c2c);
    let dir2 = (c2r - c1r, c2c - c1c);
    let pa1 = ((c1r + dir1.0) as usize, (c1c + dir1.1) as usize);
    let pa2 = ((c2r + dir2.0) as usize, (c2c + dir2.1) as usize);
    let c_iter = 0..=max_col;
    let r_iter = 0..=max_row;

    match (
        r_iter.contains(&pa1.0) && c_iter.contains(&pa1.1),
        r_iter.contains(&pa2.0) && c_iter.contains(&pa2.1),
    ) {
        (true, true) => (Some(pa1), Some(pa2)),
        (true, false) => (Some(pa1), None),
        (false, true) => (None, Some(pa2)),
        (false, false) => (None, None),
    }
}

fn count_antinodes(
    freq_map: HashMap<char, Vec<(usize, usize)>>,
    max_row: usize,
    max_col: usize,
) -> usize {
    let mut unique_antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, coords) in freq_map {
        for combo in coords.iter().combinations(2) {
            match find_antinodes(combo[0], combo[1], max_row, max_col) {
                (Some(a1), Some(a2)) => {
                    unique_antinodes.insert(a1);
                    unique_antinodes.insert(a2);
                }
                (Some(a1), None) => {
                    unique_antinodes.insert(a1);
                }
                (None, Some(a2)) => {
                    unique_antinodes.insert(a2);
                }
                (None, None) => {}
            }
        }
    }

    unique_antinodes.len()
}

fn process(input: &str) -> usize {
    let mut max_row = 0;
    let mut max_col = 0;

    let freq_map: HashMap<char, Vec<(usize, usize)>> =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut map, (r, l)| {
                for (c, ch) in l.chars().enumerate() {
                    max_col = max_col.max(c);

                    if !ch.is_alphanumeric() {
                        continue;
                    }

                    if let Some(coords) = map.get_mut(&ch) {
                        coords.push((r, c));
                    } else {
                        map.insert(ch, vec![(r, c)]);
                    }
                }

                max_row = max_row.max(r);

                map
            });

    count_antinodes(freq_map, max_row, max_col)
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

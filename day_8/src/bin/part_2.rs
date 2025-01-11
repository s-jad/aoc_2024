use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use itertools::Itertools;

fn find_antinodes(
    c1: &(usize, usize),
    c2: &(usize, usize),
    max_row: usize,
    max_col: usize,
) -> Vec<(usize, usize)> {
    let (c1r, c1c, c2r, c2c) = (c1.0 as isize, c1.1 as isize, c2.0 as isize, c2.1 as isize);
    let dir1 = (c1r - c2r, c1c - c2c);
    let dir2 = (c2r - c1r, c2c - c1c);
    let mut pa1 = ((c1r + dir1.0) as usize, (c1c + dir1.1) as usize);
    let mut pa2 = ((c2r + dir2.0) as usize, (c2c + dir2.1) as usize);
    let c_iter = 0..=max_col;
    let r_iter = 0..=max_row;

    let mut antinodes = vec![];

    while r_iter.contains(&pa1.0) && c_iter.contains(&pa1.1) {
        antinodes.push(pa1);
        pa1 = (
            (pa1.0 as isize + dir1.0) as usize,
            (pa1.1 as isize + dir1.1) as usize,
        );
    }

    while r_iter.contains(&pa2.0) && c_iter.contains(&pa2.1) {
        antinodes.push(pa2);
        pa2 = (
            (pa2.0 as isize + dir2.0) as usize,
            (pa2.1 as isize + dir2.1) as usize,
        );
    }

    antinodes
}

fn count_antinodes(
    freq_map: HashMap<char, Vec<(usize, usize)>>,
    mut unique_antinodes: HashSet<(usize, usize)>,
    max_row: usize,
    max_col: usize,
) -> usize {
    for (_, coords) in freq_map {
        for combo in coords.iter().combinations(2) {
            let anv = find_antinodes(combo[0], combo[1], max_row, max_col);
            for an in anv {
                unique_antinodes.insert(an);
            }
        }
    }

    unique_antinodes.len()
}

fn process(input: &str) -> usize {
    let mut max_row = 0;
    let mut max_col = 0;

    let (unique_antinodes, freq_map) = input.lines().enumerate().fold(
        (HashSet::new(), HashMap::<char, Vec<(usize, usize)>>::new()),
        |(mut ua, mut map), (r, l)| {
            for (c, ch) in l.chars().enumerate() {
                max_col = max_col.max(c);

                if !ch.is_alphanumeric() {
                    continue;
                }

                ua.insert((r, c));

                if let Some(coords) = map.get_mut(&ch) {
                    coords.push((r, c));
                } else {
                    map.insert(ch, vec![(r, c)]);
                }
            }

            max_row = max_row.max(r);

            (ua, map)
        },
    );

    count_antinodes(freq_map, unique_antinodes, max_row, max_col)
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

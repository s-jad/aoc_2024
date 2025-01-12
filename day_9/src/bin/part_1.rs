use itertools::Itertools;
use std::time::Instant;

fn get_blocks(disk_map: Vec<usize>) -> Vec<usize> {
    let mut id = 0;

    disk_map
        .iter()
        .enumerate()
        .flat_map(|(i, n)| {
            let mut v = vec![];

            let c = if i % 2 == 0 {
                id += 1;
                id - 1
            } else {
                usize::MAX
            };

            for _ in 0..*n {
                v.push(c);
            }

            v
        })
        .collect_vec()
}

fn pack_blocks(blocks: Vec<usize>) -> Vec<usize> {
    let mut fidx = 0usize;
    let mut bidx = blocks.len() - 1;
    let mut packed = Vec::new();

    while fidx <= bidx {
        match (blocks[fidx] < usize::MAX, blocks[bidx] < usize::MAX) {
            (true, true) => {
                packed.push(blocks[fidx]);
                fidx += 1;
            }
            (true, false) => {
                packed.push(blocks[fidx]);
                fidx += 1;
                bidx -= 1;
            }
            (false, true) => {
                packed.push(blocks[bidx]);
                fidx += 1;
                bidx -= 1;
            }
            (false, false) => {
                bidx -= 1;
            }
        }
    }

    packed
}

fn count_checksum(packed: Vec<usize>) -> usize {
    let mut total = 0;

    for (i, n) in packed.into_iter().enumerate() {
        total += n * i;
    }

    total
}

fn process(input: &str) -> usize {
    let disk_map = input.trim().chars().map(|c| c as usize - 48).collect_vec();
    let blocks = get_blocks(disk_map);
    let packed = pack_blocks(blocks);
    count_checksum(packed)
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

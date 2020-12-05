use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Copy, Clone, Debug)]
struct BoardingPass {
    pub row: usize,
    pub seat: usize,
    pub id: usize,
}

impl BoardingPass {
    pub fn decode(s: &str) -> Self {
        let row = Self::decode_fragment(127, &s[..7]);
        let seat = Self::decode_fragment(7, &s[7..]);
        let id = row * 8 + seat;

        Self { row, seat, id }
    }

    fn decode_fragment(max: usize, fragment: &str) -> usize {
        fragment
            .chars()
            .enumerate()
            .fold(max, |current, (i, char)| match char {
                'F' | 'L' => current - 2usize.pow((fragment.len() - 1 - i) as u32),
                _ => current,
            })
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<BoardingPass> {
    input
        .lines()
        .map(|line| BoardingPass::decode(line))
        .collect()
}

#[aoc(day5, part1)]
fn solve_part_1(passes: &[BoardingPass]) -> usize {
    passes.iter().map(|pass| pass.id).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part_2(passes: &[BoardingPass]) -> usize {
    let mut ids: Vec<usize> = passes.iter().map(|pass| pass.id).collect();
    ids.sort_unstable();
    let min = *ids.first().unwrap();
    let max = *ids.last().unwrap();
    let id_set: HashSet<usize> = HashSet::from_iter(ids);

    for id in min..max {
        if !id_set.contains(&id) {
            return id;
        }
    }

    panic!("no result found");
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    fn test_decode() {
        let pass = BoardingPass::from_str("FBFBBFFRLR").unwrap();

        assert_eq!(
            Pass {
                row: 44,
                seat: 5,
                id: 357
            },
            pass
        );
    }
}

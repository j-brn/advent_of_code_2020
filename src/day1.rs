use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools as _;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

/// find a pair of numbers that sums up to `sum`
fn find_pair(mut numbers: Vec<u32>, sum: u32) -> Option<(u32, u32)> {
    let (mut l, mut r) = (0, numbers.len() - 1);
    numbers.sort_unstable();

    while l < r {
        match (numbers[l], numbers[r]) {
            (a, b) if a + b == sum => return Some((a, b)),
            (a, b) if a + b < sum => l += 1,
            _ => r -= 1,
        }
    }

    None
}

/// Find a set of `n_summands` that sums up to `sum` by bruteforcing them.
fn find_summands(numbers: &[u32], n_summands: usize, sum: u32) -> Option<Vec<u32>> {
    let min = numbers.iter().copied().min().unwrap_or(u32::min_value());

    numbers
        .iter()
        .copied()
        .filter(|x| *x < sum - min)
        .combinations(n_summands)
        .find(|x| x.iter().sum::<u32>() == sum)
}

#[aoc(day1, part1)]
pub fn solve_part_1(numbers: &[u32]) -> u32 {
    let (a, b) = find_pair(numbers.to_vec(), 2020).expect("no solution found");

    a * b
}

#[aoc(day1, part2)]
pub fn solve_part_2(numbers: &[u32]) -> u32 {
    find_summands(numbers, 3, 2020)
        .expect("no solution found")
        .iter()
        .product()
}

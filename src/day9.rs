use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_invalid(numbers: &[u64]) -> Option<u64> {
    for (index, number) in numbers.iter().copied().enumerate().skip(25) {
        let valid = numbers[(index - 25)..index]
            .iter()
            .tuple_combinations()
            .any(|(a, b)| a + b == number);

        if !valid {
            return Some(number);
        }
    }

    None
}

fn find_contiguous_group(numbers: &[u64], target_sum: u64) -> Option<(usize, usize)> {
    let (mut group_start, mut group_end, mut sum) = (0, 0, 0);

    while group_end < numbers.len() {
        if sum == target_sum {
            return Some((group_start, group_end));
        }

        if sum > target_sum {
            sum -= numbers[group_start];
            group_start += 1;
        } else {
            sum += numbers[group_end];
            group_end += 1;
        }
    }

    None
}

#[aoc(day9, part1)]
fn solve_part_1(numbers: &[u64]) -> u64 {
    find_invalid(numbers).unwrap()
}

#[aoc(day9, part2)]
fn solve_part_2(numbers: &[u64]) -> u64 {
    let invalid_number = find_invalid(numbers).unwrap();
    let (group_start, group_end) = find_contiguous_group(numbers, invalid_number).unwrap();
    let mut group = numbers[group_start..=group_end].to_vec();
    group.sort_unstable();

    group.first().unwrap() + group.last().unwrap()
}

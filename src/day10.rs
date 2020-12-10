use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Adapter = u64;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Adapter> {
    let mut adapters: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

    // add power source
    adapters.push(0);
    adapters.sort_unstable();
    // add built in adapter
    adapters.push(adapters.last().unwrap() + 3);

    adapters
}

fn get_deltas(adapters: &[Adapter]) -> Vec<u64> {
    adapters
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

#[aoc(day10, part1)]
fn solve_part_1(adapters: &[Adapter]) -> usize {
    let deltas = get_deltas(adapters)
        .iter()
        .fold((0, 0), |mut deltas, delta| {
            match delta {
                1 => deltas.0 += 1,
                3 => deltas.1 += 1,
                _ => {}
            }

            deltas
        });

    deltas.0 * deltas.1
}

#[aoc(day10, part2)]
fn solve_part_2(adapters: &[Adapter]) -> u64 {
    let mut adapter_map: HashMap<u64, u64> = HashMap::with_capacity(adapters.len());
    adapter_map.insert(*adapters.last().unwrap() + 3, 1);

    for adapter in adapters.iter().copied().rev() {
        let with_delta_1 = adapter_map.get(&(adapter + 1)).copied().unwrap_or(0);
        let with_delta_2 = adapter_map.get(&(adapter + 2)).copied().unwrap_or(0);
        let with_delta_3 = adapter_map.get(&(adapter + 3)).copied().unwrap_or(0);

        adapter_map.insert(adapter, with_delta_1 + with_delta_2 + with_delta_3);
    }

    adapter_map.get(&0).copied().unwrap()
}

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::str::FromStr;

struct Group {
    group_size: usize,
    answers: HashMap<char, usize>,
}

impl Group {
    pub fn questions_answered(&self) -> usize {
        self.answers.len()
    }

    pub fn questions_answered_by_all(&self) -> usize {
        self.answers
            .values()
            .copied()
            .filter(|count| *count == self.group_size)
            .count()
    }
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let answers = s
            .chars()
            .filter(|c| c.is_alphabetic())
            .fold(HashMap::new(), |mut map, c| {
                if let Some(count) = map.get_mut(&c) {
                    *count += 1;
                } else {
                    map.insert(c, 1);
                }

                map
            });

        Ok(Self {
            group_size: s.lines().count(),
            answers,
        })
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|multiline| multiline.parse::<Group>())
        .collect::<Result<Vec<Group>, ()>>()
        .unwrap()
}

#[aoc(day6, part1)]
fn solve_part_1(groups: &[Group]) -> usize {
    groups.iter().map(|group| group.questions_answered()).sum()
}

#[aoc(day6, part2)]
fn solve_part_2(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|group| group.questions_answered_by_all())
        .sum()
}

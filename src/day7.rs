use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Rules {
    rule_map: HashMap<String, HashMap<String, usize>>,
}

impl Rules {
    pub fn bag_can_contain(&self, outer: &str, inner: &str) -> bool {
        self.rule_map
            .get(outer)
            .map(|rules| {
                rules
                    .iter()
                    .any(|(color, _count)| color == inner || self.bag_can_contain(color, inner))
            })
            .unwrap_or(false)
    }

    pub fn count_contained_bags(&self, color: &str) -> usize {
        self.rule_map
            .get(color)
            .map(|rules| {
                rules
                    .iter()
                    .map(|(color, count)| count * self.count_contained_bags(color))
                    .sum()
            })
            .unwrap_or(0)
            + 1
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Rules {
    let rule_map = input
        .lines()
        .map(|line| &line[..line.len() - 1])
        .map(|line| {
            let parts: Vec<&str> = line.split("bags contain").map(|p| p.trim()).collect();
            let color = parts.get(0).unwrap().to_string();

            let content = match *parts.get(1).unwrap() {
                "no other bags" => HashMap::new(),
                s => s
                    .split(',')
                    .map(|p| p.trim())
                    .map(|s| {
                        let parts: Vec<&str> = s.split_whitespace().map(|p| p.trim()).collect();
                        let count = parts.get(0).unwrap().parse::<usize>().unwrap();
                        let color: String = parts[1..=2].join(" ");

                        (color, count)
                    })
                    .collect(),
            };

            (color, content)
        })
        .collect();

    Rules { rule_map }
}

#[aoc(day7, part1)]
fn solve_part_1(rules: &Rules) -> usize {
    rules
        .rule_map
        .keys()
        .filter(|color| rules.bag_can_contain(color, "shiny gold"))
        .count()
}

#[aoc(day7, part2)]
fn solve_part_2(rules: &Rules) -> usize {
    rules.count_contained_bags("shiny gold") - 1
}

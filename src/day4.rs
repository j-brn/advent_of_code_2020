use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref COLOR_PATTERN: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
}

type Rule = dyn Fn(&str) -> bool;
struct PropertyValidator(HashMap<&'static str, Vec<Box<Rule>>>);

impl PropertyValidator {
    pub fn new() -> Self {
        PropertyValidator(HashMap::new())
    }

    pub fn add_rule<R>(&mut self, key: &'static str, rule: R)
    where
        R: Fn(&str) -> bool + 'static,
    {
        if !self.0.contains_key(key) {
            self.0.insert(key, Vec::new());
        }

        self.0.get_mut(key).unwrap().push(Box::new(rule));
    }

    pub fn validate_property(&self, key: &str, value: &str) -> bool {
        match self.0.get(key) {
            Some(rules) => rules.iter().all(|rule| rule(value.trim())),
            None => true,
        }
    }

    fn is_number_between(input: &str, min: u32, max: u32) -> bool {
        match input.parse::<u32>() {
            Ok(num) => num >= min && num <= max,
            Err(_) => false,
        }
    }

    fn is_one_of(input: &str, options: Vec<&str>) -> bool {
        options.contains(&input)
    }

    fn is_color_code(input: &str) -> bool {
        COLOR_PATTERN.is_match(input)
    }
}

impl Default for PropertyValidator {
    fn default() -> Self {
        let mut validator = Self::new();

        validator.add_rule("byr", |s| s.len() == 4);
        validator.add_rule("byr", |s| {
            PropertyValidator::is_number_between(s, 1920, 2002)
        });

        validator.add_rule("iyr", |s| s.len() == 4);
        validator.add_rule("iyr", |s| {
            PropertyValidator::is_number_between(s, 2010, 2020)
        });

        validator.add_rule("eyr", |s| s.len() == 4);
        validator.add_rule("eyr", |s| {
            PropertyValidator::is_number_between(s, 2020, 2030)
        });

        validator.add_rule("hgt", |s| {
            (s.ends_with("cm") && PropertyValidator::is_number_between(&s[..s.len() - 2], 150, 193))
                || (s.ends_with("in")
                    && PropertyValidator::is_number_between(&s[..s.len() - 2], 59, 76))
        });

        validator.add_rule("hcl", PropertyValidator::is_color_code);

        validator.add_rule("ecl", |s| {
            PropertyValidator::is_one_of(s, vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
        });

        validator.add_rule("pid", |s| s.len() == 9);
        validator.add_rule("pid", |s| s.parse::<u32>().is_ok());

        validator
    }
}

#[derive(Debug)]
struct Passport {
    properties: HashMap<String, String>,
}

impl Passport {
    fn contains_required_properties(&self) -> bool {
        let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        required_keys
            .iter()
            .all(|key| self.properties.contains_key(&key.to_string()))
    }

    fn is_valid(&self, validator: &PropertyValidator) -> bool {
        self.contains_required_properties()
            && self
                .properties
                .iter()
                .all(|(key, value)| validator.validate_property(key, value))
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|multiline| {
            let properties =
                multiline
                    .lines()
                    .map(|line| line.trim())
                    .fold(HashMap::new(), |mut map, line| {
                        let parts: Vec<&str> = line.split(|c| c == ':' || c == ' ').collect();

                        for pair in parts.chunks(2) {
                            let key = pair.get(0).map(|s| s.to_string()).unwrap();
                            let value = pair.get(1).map(|s| s.to_string()).unwrap();

                            map.insert(key, value);
                        }

                        map
                    });

            Passport { properties }
        })
        .collect()
}

#[aoc(day4, part1)]
fn solve_part_1(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|pass| pass.contains_required_properties())
        .count()
}

#[aoc(day4, part2)]
fn solve_part_2(input: &[Passport]) -> usize {
    let validator = PropertyValidator::default();

    input
        .iter()
        .filter(|pass| pass.is_valid(&validator))
        .count()
}

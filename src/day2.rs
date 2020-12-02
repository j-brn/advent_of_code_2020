use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq)]
struct Policy {
    position_one: usize,
    position_two: usize,
    character: char,
}

type Password = String;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<(Policy, Password)> {
    input
        .lines()
        .map(|line| {
            let parts = line
                .split(|c| c == ' ' || c == '-' || c == ':')
                .collect::<Vec<&str>>();

            let policy = Policy {
                position_one: parts.get(0).unwrap().parse().unwrap(),
                position_two: parts.get(1).unwrap().parse().unwrap(),
                character: parts.get(2).unwrap().parse().unwrap(),
            };
            (policy, parts.last().unwrap().to_string())
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part_1(input: &[(Policy, Password)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| {
            let count = password.chars().filter(|c| *c == policy.character).count();

            count >= policy.position_one && count <= policy.position_two
        })
        .count()
}

#[aoc(day2, part2)]
fn solve_part_2(input: &[(Policy, Password)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| {
            let chars = password.chars().collect::<Vec<char>>();
            let pos1 = chars.get(policy.position_one - 1).unwrap();
            let pos2 = chars.get(policy.position_two - 1).unwrap();

            (*pos1 == policy.character || *pos2 == policy.character) && pos1 != pos2
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    lazy_static! {
        static ref INPUT: Vec<(Policy, Password)> = vec![
            (Policy { position_one: 1, position_two: 3, character: 'a'}, "abcde".to_string()),
            (Policy { position_one: 1, position_two: 3, character: 'b'}, "cdefg".to_string()),
            (Policy { position_one: 2, position_two: 9, character: 'c'}, "ccccccccc".to_string()),
        ];
    }

    #[test]
    fn test_input_generator() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

        assert_eq!(input_generator(input), INPUT.as_slice());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(2, solve_part_1(INPUT.as_slice()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1, solve_part_2(INPUT.as_slice()));
    }
}

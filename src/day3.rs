use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Square {
    Tree,
    Open,
}

impl Square {
    // check if the square is occupied by a tree.
    pub fn is_tree(self) -> bool {
        self == Square::Tree
    }

    pub fn from_char(c: char) -> Result<Self, ()> {
        match c {
            '.' => Ok(Self::Open),
            '#' => Ok(Self::Tree),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Slope {
    right: usize,
    down: usize,
}

#[derive(Debug, Clone)]
struct Map {
    squares: Vec<Vec<Square>>,
}

impl Map {
    pub fn square_at(&self, x: usize, y: usize) -> Option<Square> {
        self.squares
            .get(y)
            .map(|row| row.iter().cycle().nth(x))
            .map(|opt| opt.copied())
            .flatten()
    }

    /// "travels" over the map using the given slope and returns all squares that were visited.
    pub fn travel(&self, slope: Slope) -> Vec<Square> {
        let (mut x, mut y) = (0, 0);
        let mut visited = Vec::new();

        while let Some(square) = self.square_at(x, y) {
            visited.push(square);

            x += slope.right;
            y += slope.down;
        }

        visited
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Map {
    let squares = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Square::from_char(c).unwrap())
                .collect()
        })
        .collect();

    Map { squares }
}

#[aoc(day3, part1)]
fn solve_part_1(input: &Map) -> usize {
    input
        .travel(Slope { right: 3, down: 1 })
        .iter()
        .filter(|square| square.is_tree())
        .count()
}

#[aoc(day3, part2)]
fn solve_part_2(input: &Map) -> usize {
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    slopes
        .iter()
        .copied()
        .map(|slope| {
            input
                .travel(slope)
                .iter()
                .filter(|square| square.is_tree())
                .count()
        })
        .product()
}

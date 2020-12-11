use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::fmt::{self, Display};

type Position = (i32, i32);
type Vector = Position;

#[derive(Debug)]
struct Ferry {
    seats: HashMap<Position, State>,
}

const fn neighbour_coordinates_and_vectors((row, col): Position) -> [(Position, Vector); 8] {
    [
        ((row - 1, col - 1), (-1, -1)),
        ((row, col - 1), (0, -1)),
        ((row + 1, col - 1), (1, -1)),
        ((row - 1, col), (-1, 0)),
        ((row + 1, col), (1, 0)),
        ((row - 1, col + 1), (-1, 1)),
        ((row, col + 1), (0, 1)),
        ((row + 1, col + 1), (1, 1)),
    ]
}

impl Ferry {
    pub fn new(seats: HashMap<Position, State>) -> Self {
        Self { seats }
    }

    pub fn occupy_all(&mut self) {
        self.seats
            .values_mut()
            .filter(|state| **state != State::Blocked)
            .for_each(|state| *state = State::Occupied)
    }

    fn neighbours(&self, position: Position) -> Vec<(Position, State)> {
        let neighbour_coordinates = neighbour_coordinates_and_vectors(position);

        neighbour_coordinates
            .iter()
            .filter_map(|(coordinates, _)| {
                self.seats
                    .get(coordinates)
                    .copied()
                    .map(|state| (*coordinates, state))
            })
            .collect()
    }

    fn next_visible_seats(&self, position: Position) -> Vec<(Position, State)> {
        let neighbour_coordinates = neighbour_coordinates_and_vectors(position);

        neighbour_coordinates
            .iter()
            .filter_map(|((mut row, mut col), v)| {
                while let Some(state) = self.seats.get(&(row, col)) {
                    if *state != State::Blocked {
                        return Some(((row, col), *state));
                    }

                    row += v.0;
                    col += v.1;
                }

                None
            })
            .collect()
    }

    fn next_state1(&self, (position, state): (Position, State)) -> State {
        let occupied_neighbours = self
            .neighbours(position)
            .iter()
            .filter(|(_, state)| *state == State::Occupied)
            .count();

        match (state, occupied_neighbours) {
            (State::Empty, 0) => State::Occupied,
            (State::Occupied, n) if n >= 4 => State::Empty,
            (state, _n) => state,
        }
    }

    fn next_state2(&self, (position, state): (Position, State)) -> State {
        let visible_occupied_seats = self
            .next_visible_seats(position)
            .iter()
            .filter(|(_, state)| *state == State::Occupied)
            .count();

        match (state, visible_occupied_seats) {
            (State::Empty, 0) => State::Occupied,
            (State::Occupied, n) if n >= 5 => State::Empty,
            (state, _n) => state,
        }
    }

    fn run_cycle1(&mut self) -> bool {
        let mut changed = false;

        let new_states: Vec<(Position, State)> = self
            .seats
            .iter()
            .map(|(coordinates, state)| {
                let new_state = self.next_state1((*coordinates, *state));

                if *state != new_state {
                    changed = true;
                }

                (*coordinates, new_state)
            })
            .collect();

        for (position, state) in new_states {
            self.seats.insert(position, state);
        }

        changed
    }

    fn run_cycle2(&mut self) -> bool {
        let mut changed = false;

        let new_states: Vec<(Position, State)> = self
            .seats
            .iter()
            .map(|(coordinates, state)| {
                let new_state = self.next_state2((*coordinates, *state));

                if *state != new_state {
                    changed = true;
                }

                (*coordinates, new_state)
            })
            .collect();

        for (position, state) in new_states {
            self.seats.insert(position, state);
        }

        changed
    }

    fn count_occupied_seats(&self) -> usize {
        self.seats
            .values()
            .copied()
            .filter(|state| *state == State::Occupied)
            .count()
    }

    #[allow(dead_code)]
    fn print(&self, rows: i32, columns: i32) {
        print!("\n");
        for row in 0..rows {
            for column in 0..columns {
                let s = self
                    .seats
                    .get(&(row, column))
                    .map(|state| state.to_string())
                    .unwrap_or(" ".to_string());

                print!("{}", s);
            }
            print!("\n");
        }
        print!("\n");
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Empty,
    Occupied,
    Blocked,
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Empty => 'L',
                State::Occupied => '#',
                State::Blocked => '.',
            }
        )
    }
}

impl State {
    pub fn from_char(c: char) -> Result<Self, ()> {
        match c {
            '#' => Ok(Self::Occupied),
            'L' => Ok(Self::Empty),
            '.' => Ok(Self::Blocked),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> HashMap<(i32, i32), State> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| ((row as i32, col as i32), State::from_char(char).unwrap()))
                .collect::<Vec<((i32, i32), State)>>()
        })
        .flatten()
        .fold(HashMap::new(), |mut map, (coordinates, state)| {
            map.insert(coordinates, state);
            map
        })
}

#[aoc(day11, part1)]
fn solve_part_1(seats: &HashMap<(i32, i32), State>) -> usize {
    let mut ferry = Ferry::new(seats.clone());
    ferry.occupy_all();

    loop {
        if !ferry.run_cycle1() {
            break;
        }
    }

    ferry.count_occupied_seats()
}

#[aoc(day11, part2)]
fn solve_part_2(seats: &HashMap<(i32, i32), State>) -> usize {
    let mut ferry = Ferry::new(seats.clone());
    ferry.occupy_all();

    loop {
        if !ferry.run_cycle2() {
            break;
        }
    }

    ferry.count_occupied_seats()
}

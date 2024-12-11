use anyhow::Result;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Empty,
    Wall,
    Visited,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

struct Board {
    width: usize,
    height: usize,
    data: Vec<State>,
}

impl Board {
    fn from_str(input: &str) -> Self {
        let data = input
            .lines()
            .flat_map(|line| line.bytes())
            .map(|byte| match byte {
                b'.' => State::Empty,
                b'#' => State::Wall,
                b'^' => State::Visited,
                _ => panic!("Invalid byte {}", byte),
            })
            .collect::<Vec<_>>();

        let width = input.find('\n').unwrap();
        let height = data.len() / width;

        Self {
            width,
            height,
            data,
        }
    }

    fn get(&self, loc: &Coordinate) -> Option<State> {
        if loc.row >= self.height || loc.col >= self.width {
            return None;
        }
        self.data.get(loc.row * self.width + loc.col).copied()
    }

    fn set(&mut self, loc: &Coordinate, state: State) {
        self.data[loc.row * self.width + loc.col] = state;
    }

    fn find_start(&self) -> Coordinate {
        self.data
            .iter()
            .enumerate()
            .find_map(|(i, &state)| {
                (state == State::Visited).then(|| Coordinate::new(i / self.width, i % self.width))
            })
            .unwrap()
    }

    fn walk_to_wall(&mut self, guard: &Guard) -> Option<Coordinate> {
        let mut loc = guard.loc;

        let mut prev = loc;
        loop {
            match self.get(&loc) {
                Some(State::Wall) => return Some(prev),
                Some(State::Empty) => {
                    self.set(&loc, State::Visited);
                }
                Some(State::Visited) => {}
                None => return None,
            }

            prev = loc;
            match guard.direction {
                Direction::Up => loc.row = loc.row.checked_sub(1)?,
                Direction::Down => loc.row += 1,
                Direction::Left => loc.col = loc.col.checked_sub(1)?,
                Direction::Right => loc.col += 1,
            }
        }
    }
}

struct Guard {
    loc: Coordinate,
    direction: Direction,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut board = Board::from_str(&input);

    let mut guard = Guard {
        loc: board.find_start(),
        direction: Direction::Up,
    };

    while let Some(new_loc) = board.walk_to_wall(&guard) {
        let new_direction = match guard.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };

        guard.loc = new_loc;
        guard.direction = new_direction;
    }

    let result = board
        .data
        .iter()
        .filter(|&&state| state == State::Visited)
        .count();

    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {}

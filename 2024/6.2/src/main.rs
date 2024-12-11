use anyhow::Result;
use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: isize,
    col: isize,
}

impl Coordinate {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone)]
struct Board {
    width: usize,
    height: usize,
    data: Vec<State>,
}

impl Board {
    fn from_str(input: &str) -> (Self, Guard) {
        let width = input.find('\n').unwrap();

        let mut guard_loc = None;
        let data = input
            .lines()
            .flat_map(|line| line.bytes())
            .enumerate()
            .map(|(i, byte)| match byte {
                b'.' => State::Empty,
                b'#' => State::Wall,
                b'^' => {
                    guard_loc = Some(Coordinate::new((i / width) as isize, (i % width) as isize));
                    State::Empty
                }
                _ => panic!("Invalid byte {}", byte as char),
            })
            .collect::<Vec<_>>();

        let width = input.find('\n').unwrap();
        let height = data.len() / width;

        (
            Self {
                width,
                height,
                data,
            },
            Guard::new(guard_loc.unwrap(), Direction::Up),
        )
    }

    fn get(&self, loc: &Coordinate) -> Option<State> {
        if !self.contains(loc) {
            return None;
        }
        Some(self.data[loc.row as usize * self.width + loc.col as usize])
    }

    fn set(&mut self, loc: &Coordinate, state: State) {
        if !self.contains(loc) {
            panic!("Invalid location {:?}", loc);
        }
        self.data[loc.row as usize * self.width + loc.col as usize] = state;
    }

    fn contains(&self, loc: &Coordinate) -> bool {
        (0..self.height as isize).contains(&loc.row) && (0..self.width as isize).contains(&loc.col)
    }

    fn with_additional_wall(&self, loc: &Coordinate) -> Self {
        let mut new_board = self.clone();
        new_board.set(loc, State::Wall);
        new_board
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    loc: Coordinate,
    direction: Direction,
}

impl Guard {
    fn new(loc: Coordinate, direction: Direction) -> Self {
        Self { loc, direction }
    }

    fn next_loc(&self) -> Coordinate {
        let mut loc = self.loc;
        match self.direction {
            Direction::Up => loc.row -= 1,
            Direction::Down => loc.row += 1,
            Direction::Left => loc.col -= 1,
            Direction::Right => loc.col += 1,
        }
        loc
    }

    fn next_direction(&self) -> Direction {
        self.direction.next()
    }

    fn next_state(&self) -> Self {
        Guard::new(self.next_loc(), self.direction)
    }

    fn turn(&mut self) {
        self.direction = self.next_direction();
    }

    fn move_forward(&mut self) {
        self.loc = self.next_loc();
    }
}

#[derive(Debug, Clone)]
struct Game {
    board: Board,
    guard: Guard,
}

impl Game {
    fn new(board: Board, guard: Guard) -> Self {
        Self { board, guard }
    }

    fn from_str(input: &str) -> Self {
        let (board, guard) = Board::from_str(input);
        Self::new(board, guard)
    }

    fn count_loops(&mut self) -> HashSet<Coordinate> {
        let orig_guard = self.guard;
        let mut new_obstacles = HashSet::new();
        loop {
            // Inspect the next location
            let next_loc = self.guard.next_loc();
            match self.board.get(&next_loc) {
                Some(State::Wall) => self.guard.turn(),
                Some(State::Empty) => {
                    let tmp_board = self.board.with_additional_wall(&next_loc);
                    let mut tmp_game = Game::new(tmp_board, orig_guard);
                    if tmp_game.has_loop() {
                        new_obstacles.insert(next_loc);
                    }

                    self.guard.move_forward();
                }
                None => return new_obstacles,
            }
        }
    }

    fn has_loop(&mut self) -> bool {
        let mut visited_turns = HashSet::new();
        loop {
            // Inspect the next location
            let next = self.guard.next_state();
            match self.board.get(&next.loc) {
                Some(State::Wall) => {
                    // If we turn twice in the same location with the same direction,
                    // we have a loop
                    if !visited_turns.insert(next) {
                        return true;
                    }
                    self.guard.turn();
                }
                Some(State::Empty) => self.guard.move_forward(),
                None => return false,
            }
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut game = Game::from_str(&input);

    let result = game.count_loops();

    println!("{}", result.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_count_loops() {
        let mut game = Game::from_str(INPUT1);

        let result = game.count_loops();

        assert_eq!(result.len(), 6);
    }

    const INPUT2: &str = r#"............#................##.
...............................#
................................
.^..............................
.....................#..........
..#.................#...........
...................#............
................................
................................
................................
........#......#.......#........
.......#........................
................................
..........#...........#.........
..........#.....#.....##........
................................
............................#..#
................................
.............................#..
.................#.....#......#."#;

    #[test]
    fn test_count_special_loops() {
        let mut game = Game::from_str(INPUT2);

        let result = game.count_loops();

        assert_eq!(result.len(), 1);
    }
}

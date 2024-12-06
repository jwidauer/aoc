use std::fs::{self};

use anyhow::Result;

struct Puzzle {
    pub input: String,
    pub cols: usize,
    pub rows: usize,
}

impl Puzzle {
    fn from_input(input: String) -> Self {
        let cols = input.find('\n').unwrap();
        let rows = input.len() / (cols + 1);
        Self { input, cols, rows }
    }

    fn get(&self, row: usize, col: usize) -> Option<char> {
        self.input
            .as_bytes()
            .get(row * (self.cols + 1) + col)
            .map(|&c| c as char)
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn match_coords(&self, c: char) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.input
            .match_indices(c)
            .map(|(i, _)| (i / (self.cols + 1), i % (self.cols + 1)))
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let puzzle = Puzzle::from_input(input);

    let coords: Vec<_> = puzzle
        .match_coords('A')
        .filter(|(r, c)| (1..puzzle.rows() - 1).contains(r) && (1..puzzle.cols() - 1).contains(c))
        .collect();

    let mut count = 0;
    for (r, c) in coords {
        let top_left = puzzle.get(r - 1, c - 1).unwrap();
        let top_right = puzzle.get(r - 1, c + 1).unwrap();
        let bottom_left = puzzle.get(r + 1, c - 1).unwrap();
        let bottom_right = puzzle.get(r + 1, c + 1).unwrap();

        if ((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M'))
            && ((top_right == 'M' && bottom_left == 'S')
                || (top_right == 'S' && bottom_left == 'M'))
        {
            count += 1;
        }
    }

    println!("Count: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let input = "ABC\nDEF\nGHI".to_string();
        let puzzle = Puzzle::from_input(input);

        assert_eq!(puzzle.get(0, 0), Some('A'));
        assert_eq!(puzzle.get(0, 1), Some('B'));
        assert_eq!(puzzle.get(0, 2), Some('C'));
        assert_eq!(puzzle.get(1, 0), Some('D'));
        assert_eq!(puzzle.get(1, 1), Some('E'));
        assert_eq!(puzzle.get(1, 2), Some('F'));
        assert_eq!(puzzle.get(2, 0), Some('G'));
        assert_eq!(puzzle.get(2, 1), Some('H'));
        assert_eq!(puzzle.get(2, 2), Some('I'));
    }

    #[test]
    fn test_match_coords() {
        let input = "ABC\nDAF\nGHI".to_string();
        let puzzle = Puzzle::from_input(input);

        let coords: Vec<_> = puzzle.match_coords('A').collect();
        assert_eq!(coords, vec![(0, 0), (1, 1)]);

        let coords: Vec<_> = puzzle.match_coords('B').collect();
        assert_eq!(coords, vec![(0, 1)]);

        let coords: Vec<_> = puzzle.match_coords('C').collect();
        assert_eq!(coords, vec![(0, 2)]);

        let coords: Vec<_> = puzzle.match_coords('D').collect();
        assert_eq!(coords, vec![(1, 0)]);

        let coords: Vec<_> = puzzle.match_coords('F').collect();
        assert_eq!(coords, vec![(1, 2)]);

        let coords: Vec<_> = puzzle.match_coords('G').collect();
        assert_eq!(coords, vec![(2, 0)]);

        let coords: Vec<_> = puzzle.match_coords('H').collect();
        assert_eq!(coords, vec![(2, 1)]);
    }
}

use std::fs::{self};

use anyhow::Result;
use regex::Regex;

fn count_occurences(input: &str) -> usize {
    let re = Regex::new(r"XMAS").unwrap();
    let rev = Regex::new(r"SAMX").unwrap();

    re.find_iter(input).count() + rev.find_iter(input).count()
}

fn transpose(input: &str, line_len: usize, nr_lines: usize) -> String {
    let input = input.as_bytes();
    let mut transposed = String::with_capacity(input.len());
    for col in 0..line_len {
        for row in 0..nr_lines {
            transposed.push(*input.get(row * (line_len + 1) + col).unwrap() as char);
        }
        transposed.push('\n');
    }
    transposed
}

fn get_diagonals(input: &str, line_len: usize, nr_lines: usize) -> String {
    let input = input.as_bytes();
    let mut diagonals = String::with_capacity(input.len() + nr_lines);
    let nr_diagonals = nr_lines + line_len - 1;
    let nr_lines = nr_lines as isize;

    for i in 1..=nr_diagonals {
        let i = i as isize;
        let mut nr_elements = (i).min(nr_lines) + (nr_lines - i).min(0);
        let mut row = (i - 1).min(nr_lines - 1);
        let mut col = (i - nr_lines).max(0);
        while nr_elements > 0 {
            let idx = row * (line_len + 1) as isize + col;
            diagonals.push(*input.get(idx as usize).unwrap() as char);
            row -= 1;
            col += 1;
            nr_elements -= 1;
        }
        diagonals.push('\n');
    }

    diagonals
}

fn get_anti_diagonals(input: &str, line_len: usize, nr_lines: usize) -> String {
    let input = input.as_bytes();
    let mut diagonals = String::with_capacity(input.len() + nr_lines);
    let nr_diagonals = nr_lines + line_len - 1;
    let nr_lines = nr_lines as isize;

    for i in 1..=nr_diagonals {
        let i = i as isize;
        let mut nr_elements = (i).min(nr_lines) + (nr_lines - i).min(0);
        let mut row = (nr_lines - i).max(0);
        let mut col = (i - nr_lines).max(0);
        while nr_elements > 0 {
            let idx = row * (line_len + 1) as isize + col;
            diagonals.push(*input.get(idx as usize).unwrap() as char);
            row += 1;
            col += 1;
            nr_elements -= 1;
        }
        diagonals.push('\n');
    }

    diagonals
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let line_len = input.find('\n').unwrap();
    let row_len = line_len + 1;
    let nr_lines = input.len() / row_len;

    // Find all horizontal occurences
    let mut count = count_occurences(&input);

    // Find all vertical occurences
    let transposed = transpose(&input, line_len, nr_lines);
    count += count_occurences(&transposed);

    // Find all occurences in the diagonal input
    let diagonals = get_diagonals(&input, line_len, nr_lines);
    count += count_occurences(&diagonals);

    // Find all occurences in the anti-diagonal input
    let anti_diagonals = get_anti_diagonals(&input, line_len, nr_lines);
    count += count_occurences(&anti_diagonals);

    println!("count: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurences() {
        let input = "XMAS\nSAMX\nXMAS\nSAMX";
        assert_eq!(count_occurences(input), 4);
    }

    #[test]
    fn test_transpose() {
        let input = "012\n345\n678\n";
        let line_len = input.find('\n').unwrap();
        let row_len = line_len + 1;
        let nr_lines = input.len() / row_len;
        let transposed = transpose(input, line_len, nr_lines);
        assert_eq!(transposed, "036\n147\n258\n");
    }

    #[test]
    fn test_get_diagonals_small() {
        let input = "012\n345\n678\n";
        let line_len = input.find('\n').unwrap();
        let row_len = line_len + 1;
        let nr_lines = input.len() / row_len;
        let diagonals = get_diagonals(input, line_len, nr_lines);
        assert_eq!(diagonals, "0\n31\n642\n75\n8\n");
    }

    #[test]
    fn test_get_diagonals_large() {
        let input = "01234\n01234\n01234\n01234\n01234\n";
        let line_len = input.find('\n').unwrap();
        let row_len = line_len + 1;
        let nr_lines = input.len() / row_len;
        let diagonals = get_diagonals(input, line_len, nr_lines);
        assert_eq!(diagonals, "0\n01\n012\n0123\n01234\n1234\n234\n34\n4\n");
    }

    #[test]
    fn test_get_anti_diagonals_small() {
        let input = "012\n345\n678\n";
        let line_len = input.find('\n').unwrap();
        let row_len = line_len + 1;
        let nr_lines = input.len() / row_len;
        let diagonals = get_anti_diagonals(input, line_len, nr_lines);
        assert_eq!(diagonals, "6\n37\n048\n15\n2\n");
    }

    #[test]
    fn test_get_anti_diagonals_large() {
        let input = "01234\n01234\n01234\n01234\n01234\n";
        let line_len = input.find('\n').unwrap();
        let row_len = line_len + 1;
        let nr_lines = input.len() / row_len;
        let diagonals = get_anti_diagonals(input, line_len, nr_lines);
        assert_eq!(diagonals, "0\n01\n012\n0123\n01234\n1234\n234\n34\n4\n");
    }
}

use anyhow::{anyhow, bail, Result};
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

#[derive(Debug, Clone)]
struct Equation {
    result: u64,
    values: Vec<u64>,
}

impl Equation {
    fn new(result: u64, values: Vec<u64>) -> Self {
        Self { result, values }
    }

    fn is_possible(&self) -> bool {
        let mut operators = vec![Op::Add; self.values.len() - 1];
        let mut i = 0;

        while i < 2u64.pow(operators.len() as u32) {
            if self.result
                == self.values[1..]
                    .iter()
                    .zip(operators.iter())
                    .fold(self.values[0], |acc, (value, op)| op.apply(acc, *value))
            {
                return true;
            }

            i += 1;
            for (j, o) in operators.iter_mut().enumerate() {
                match i & (1 << j) {
                    0 => *o = Op::Add,
                    _ => *o = Op::Mul,
                }
            }
        }

        false
    }
}

impl TryFrom<&str> for Equation {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let idx = value.find(':').ok_or(anyhow!("No colon found!"))?;
        let result = value[0..idx].parse()?;
        let values = value[idx..]
            .split(' ')
            .skip(1)
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?;

        if values.is_empty() {
            bail!("No values found!");
        }

        Ok(Self::new(result, values))
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let equations = input
        .lines()
        .map(Equation::try_from)
        .collect::<Result<Vec<_>>>()?;

    let result = equations
        .iter()
        .filter(|e| e.is_possible())
        .map(|e| e.result)
        .sum::<u64>();

    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_possible() {
        let e = Equation::new(3267, vec![81, 40, 27]);
        assert!(e.is_possible());
    }

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_main() {
        let equations = INPUT
            .lines()
            .map(Equation::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();

        dbg!(&equations[0]);

        let result = equations
            .iter()
            .filter(|e| e.is_possible())
            .map(|e| e.result)
            .inspect(|r| println!("{}", r))
            .sum::<u64>();

        assert_eq!(result, 190 + 3267 + 292);
    }
}

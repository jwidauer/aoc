mod input;
use anyhow::Result;
use input::INPUT;
use regex::Regex;

fn main() -> Result<()> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let res: u64 = re
        .captures_iter(INPUT)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            let (a, b): (u64, u64) = (a.parse().unwrap(), b.parse().unwrap());
            a * b
        })
        .sum();

    println!("{}", res);

    Ok(())
}

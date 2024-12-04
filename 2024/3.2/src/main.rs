mod input;
use anyhow::Result;
use input::INPUT;
use regex::Regex;

fn main() -> Result<()> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let mut mul_enabled = true;
    let res: u64 = re
        .captures_iter(INPUT)
        .filter_map(|caps| {
            if caps.name("do").is_some() {
                mul_enabled = true;
                return None;
            }
            if caps.name("dont").is_some() {
                mul_enabled = false;
                return None;
            }

            if !mul_enabled {
                return None;
            }

            let a: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let b: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
            Some(a * b)
        })
        .sum();

    println!("{}", res);

    Ok(())
}

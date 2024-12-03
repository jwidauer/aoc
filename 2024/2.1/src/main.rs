mod input;
use anyhow::Result;

fn main() -> Result<()> {
    let res = input::INPUT
        .lines()
        .map(|l| {
            let levels: Vec<u32> = l.split(' ').map(|num| num.parse().unwrap()).collect();
            (levels.is_sorted_by(|a, b| a < b) || levels.is_sorted_by(|a, b| a > b))
                && levels
                    .as_slice()
                    .windows(2)
                    .all(|s| (1..=3).contains(&s[0].abs_diff(s[1])))
        })
        .filter(|&b| b)
        .count();

    println!("{}", res);

    Ok(())
}

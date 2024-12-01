mod input;
use anyhow::Result;

fn main() -> Result<()> {
    let (mut lnums, mut rnums): (Vec<_>, Vec<_>) = input::INPUT
        .lines()
        .map(|l| {
            let mut it = l.split("   ");
            let (lnum, rnum) = (it.next().unwrap(), it.next().unwrap());
            (lnum.parse::<i32>().unwrap(), rnum.parse::<i32>().unwrap())
        })
        .unzip();

    lnums.sort();
    rnums.sort();

    let res = lnums
        .iter()
        .zip(rnums.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());

    println!("{}", res);

    Ok(())
}

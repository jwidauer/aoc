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

    let rnums = rnums
        .iter()
        .fold(std::collections::HashMap::new(), |mut map, &rnum| {
            *map.entry(rnum).or_insert(0) += 1;
            map
        });

    let res = lnums
        .iter()
        .fold(0, |acc, &lnum| acc + rnums.get(&lnum).unwrap_or(&0) * lnum);

    println!("{}", res);

    Ok(())
}

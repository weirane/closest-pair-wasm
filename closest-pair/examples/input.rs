use closest_pair::{closest_pair, ParsePointError, Point};
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() -> Result<(), ParsePointError> {
    let points: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| Ok(Point::from_str(&line)?))
        .collect::<Result<_, ParsePointError>>()?;

    let (dist, p0, p1) = closest_pair(&points);

    println!("{:.6}", dist);
    println!("{:.6} <=> {:.6}", p0, p1);

    Ok(())
}

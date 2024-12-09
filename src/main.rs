use anyhow::anyhow;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod utils;
mod day10;

fn main() -> anyhow::Result<()> {
    let day: u16 = std::env::args()
        .nth(1)
        .ok_or(anyhow!("Missing day"))?
        .parse()?;

    let example = std::env::args().nth(2).unwrap_or_default() == "e";

    match day {
        1 => day01::run(example)?,
        2 => day02::run(example)?,
        3 => day03::run(example)?,
        4 => day04::run(example)?,
        5 => day05::run(example)?,
        6 => day06::run(example)?,
        7 => day07::run(example)?,
        8 => day08::run(example)?,
        9 => day09::run(example)?,
        10 => day10::run(example)?,
        _ => return Err(anyhow!("Unknown day")),
    }

    Ok(())
}

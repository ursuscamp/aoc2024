use anyhow::anyhow;

mod day01;
mod day02;
mod utils;

fn main() -> anyhow::Result<()> {
    let day: u16 = std::env::args()
        .nth(1)
        .ok_or(anyhow!("Missing day"))?
        .parse()?;

    let example = std::env::args().nth(2).unwrap_or_default() == "e";

    match day {
        1 => day01::run(example)?,
        2 => day02::run(example)?,
        _ => return Err(anyhow!("Unknown day")),
    }

    Ok(())
}

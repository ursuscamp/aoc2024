use std::fmt::write;

use anyhow::anyhow;

mod day1;

fn main() -> anyhow::Result<()> {
    let day: u16 = std::env::args()
        .nth(1)
        .ok_or(anyhow!("Missing day"))?
        .parse()?;

    match day {
        1 => day1::run()?,
        _ => return Err(anyhow!("Unknown day")),
    }

    Ok(())
}

pub fn input(day: u16, example: bool) -> anyhow::Result<String> {
    let mut fname = format!("inputs/{day:02}");
    if example {
        fname.push('e');
    }
    fname.push_str(".txt");
    Ok(std::fs::read_to_string(&fname)?)
}

use anyhow::Result;
use day2::{process_input, Colors};
use std::fs;

fn main() -> Result<()> {
    let f = fs::read("test.txt")?;
    let val = process_input(&f, (Colors::Red(12), Colors::Green(13), Colors::Blue(14)));

    println!("val: {val}");
    Ok(())
}

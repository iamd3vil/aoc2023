use anyhow::Result;
use day2::process_input;
use std::fs;

fn main() -> Result<()> {
    let f = fs::read("test.txt")?;
    let val = process_input(&f);

    println!("val: {val}");
    Ok(())
}

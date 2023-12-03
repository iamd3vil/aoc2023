// use crate::process_input;
use anyhow::Result;
use day1::process_input;
use std::fs;

fn main() -> Result<()> {
    let f = fs::read("test.txt")?;
    let total_value: u32 = process_input(&f);

    println!("Total value: {}", total_value);

    Ok(())
}

use anyhow::Result;
use day6::process_input;
use std::fs;

fn main() -> Result<()> {
    let cnt = fs::read("test.txt")?;
    let val = process_input(&mut &cnt)?;

    println!("val: {val}");
    Ok(())
}

// use crate::process_input;
use anyhow::Result;
use day1::process_input;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<()> {
    let f = File::open("test.txt")?;
    let mut rdr = BufReader::new(f);
    let total_value: u32 = process_input(&mut rdr);

    println!("Total value: {}", total_value);

    Ok(())
}

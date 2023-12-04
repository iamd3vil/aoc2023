use anyhow::Result;
use day3::process_input;
use std::{fs::File, io::BufReader};

fn main() -> Result<()> {
    let f = File::open("test.txt")?;
    let mut rdr = BufReader::new(f);
    let val = process_input(&mut rdr);

    println!("val: {val}");
    Ok(())
}

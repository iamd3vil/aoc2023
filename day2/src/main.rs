use anyhow::Result;
use day2::process_input;
use std::{fs::File, io::BufReader};

fn main() -> Result<()> {
    let f = File::open("test.txt")?;
    let mut rdr = BufReader::new(f);
    // let f = fs::read("test.txt")?;
    let val = process_input(&mut rdr);

    println!("val: {val}");
    Ok(())
}

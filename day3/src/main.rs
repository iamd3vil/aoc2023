use anyhow::Result;
use day3::process_input;
use std::fs;

fn main() -> Result<()> {
    // let f = File::open("test.txt")?;
    // let mut rdr = BufReader::new(f);
    let cnt = fs::read("test.txt")?;
    let val = process_input(&cnt);

    println!("val: {val}");
    Ok(())
}

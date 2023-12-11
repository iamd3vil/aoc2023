use anyhow::Result;
use std::io::BufRead;

pub fn process_input(input: &[u8]) -> Result<u64> {
    let mut lines = input.lines();
    let time_lines: String = lines.next().expect("expected first line")?;

    let times: Vec<u64> = get_digits(&time_lines, "Time:");

    let distance_lines: String = lines.next().expect("expected second line")?;

    let distances: Vec<u64> = get_digits(&distance_lines, "Distance:");

    let ways = times
        .into_iter()
        .zip(distances.into_iter())
        .map(number_of_ways_to_win)
        .product::<u64>();
    Ok(ways)
}

fn get_digits(line: &str, prefix: &str) -> Vec<u64> {
    line.strip_prefix(prefix)
        .expect("expected prefix")
        .split_whitespace()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                None
            } else {
                Some(l.parse::<u64>().expect("expected u64"))
            }
        })
        .collect()
}

fn number_of_ways_to_win((t, d): (u64, u64)) -> u64 {
    (0..t)
        .filter(|n| -> bool {
            let dist = (t - n) * n;

            dist > d
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_ways_to_win() {
        assert_eq!(number_of_ways_to_win((7, 9)), 4);
        assert_eq!(number_of_ways_to_win((15, 40)), 8);
        assert_eq!(number_of_ways_to_win((30, 200)), 9);
    }

    #[test]
    fn test_process_input() -> Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200"
            .as_bytes();

        assert_eq!(process_input(input)?, 4 * 8 * 9);

        Ok(())
    }
}

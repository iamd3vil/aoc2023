use anyhow::Result;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let total_value: u32 = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| correct_digits_in_line(&line))
        .map(|line| get_value_from_line(line))
        .sum();

    println!("Total value: {}", total_value);

    Ok(())
}

// This returns the first digit, last digit found in the line as the result.
// Example: '1abc2' -> 12
fn get_value_from_line(line: String) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;

    for c in line.chars() {
        if c.is_digit(10) {
            if first_digit == 0 {
                first_digit = c.to_digit(10).unwrap() as u32 * 10;
            }
            last_digit = c.to_digit(10).unwrap() as u32;
        }
    }

    first_digit + last_digit
}

// get_digit_from_word returns the digit representation of word so that we can
// replace that in the line.
// Example: 'one' -> '1'
fn get_digit_from_word(word: &str) -> &str {
    match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "",
    }
}

fn correct_digits_in_line(line: &str) -> String {
    let mut new_line = "".to_string();
    let len = line.len();

    'outer: for i in 0..len {
        for j in i..len {
            let w = &line[i..j + 1];

            if w.len() == 1 {
                let c = w.chars().next().unwrap();
                if c.is_digit(10) {
                    new_line.push(c);
                    continue 'outer;
                }
            }

            let digit_word = get_digit_from_word(w);

            if !digit_word.is_empty() {
                new_line.push_str(digit_word);
                continue 'outer;
            }
        }
    }

    new_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_from_line() {
        assert_eq!(get_value_from_line("1abc2".to_string()), 12);
        assert_eq!(get_value_from_line("1abc2def3".to_string()), 13);
        assert_eq!(get_value_from_line("1abc2def3ghi4".to_string()), 14);
        assert_eq!(get_value_from_line("1abc2def3ghi4jkl5".to_string()), 15);
        assert_eq!(get_value_from_line("1abc2def3ghi4jkl5mno6".to_string()), 16);
        assert_eq!(
            get_value_from_line("1abc2def3ghi4jkl5mno6pqr7".to_string()),
            17
        );
        assert_eq!(
            get_value_from_line("1abc2def3ghi4jkl5mno6pqr7stu8".to_string()),
            18
        );
        assert_eq!(
            get_value_from_line("5abc2def3ghi4jkl5mno6pqr7stu8vwx9".to_string()),
            59
        );
        assert_eq!(
            get_value_from_line("1abc2def3ghi4jkl5mno6pqr7stu8vwx9yz0".to_string()),
            10
        );
    }

    #[test]
    fn test_val_from_line() {
        let l = "eightwothree";
        assert_eq!(correct_digits_in_line(l), "823".to_string());
    }
}

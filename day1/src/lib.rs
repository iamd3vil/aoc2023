use std::io::BufRead;

pub fn process_input(lines: &Vec<u8>) -> u32 {
    lines
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| correct_digits_in_line(&line))
        .map(|line| get_value_from_line(&line))
        .sum()
}

// This returns the first digit, last digit found in the line as the result.
// Example: '1abc2' -> 12
fn get_value_from_line(line: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;

    for c in line.chars() {
        if c.is_digit(10) {
            if first_digit == 0 {
                first_digit = c.to_digit(10).unwrap() as u32;
            }
            last_digit = c.to_digit(10).unwrap() as u32;
        }
    }

    first_digit * 10 + last_digit
}

pub fn correct_digits_in_line(line: &str) -> String {
    let mut new_line = String::new();
    let len = line.len();
    for i in 0..len {
        let red_line = &line[i..];

        if red_line.starts_with("one") {
            new_line.push('1');
        } else if red_line.starts_with("two") {
            new_line.push('2');
        } else if red_line.starts_with("three") {
            new_line.push('3');
        } else if red_line.starts_with("four") {
            new_line.push('4');
        } else if red_line.starts_with("five") {
            new_line.push('5');
        } else if red_line.starts_with("six") {
            new_line.push('6');
        } else if red_line.starts_with("seven") {
            new_line.push('7');
        } else if red_line.starts_with("eight") {
            new_line.push('8');
        } else if red_line.starts_with("nine") {
            new_line.push('9');
        } else {
            let c = red_line.chars().next().unwrap();
            if c.is_digit(10) {
                new_line.push(c);
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
        assert_eq!(get_value_from_line("1abc2"), 12);
        assert_eq!(get_value_from_line("1abc2def3"), 13);
        assert_eq!(get_value_from_line("1abc2def3ghi4"), 14);
        assert_eq!(get_value_from_line("1abc2def3ghi4jkl5"), 15);
        assert_eq!(get_value_from_line("1abc2def3ghi4jkl5mno6"), 16);
        assert_eq!(get_value_from_line("1abc2def3ghi4jkl5mno6pqr7"), 17);
        assert_eq!(get_value_from_line("1abc2def3ghi4jkl5mno6pqr7stu8"), 18);
        assert_eq!(get_value_from_line("5abc2def3ghi4jkl5mno6pqr7stu8vwx9"), 59);
        assert_eq!(
            get_value_from_line("1abc2def3ghi4jkl5mno6pqr7stu8vwx9yz0"),
            10
        );
    }

    #[test]
    fn test_val_from_line() {
        let l = "xtqtwoneeightlvcjqfourckfour2nine";
        assert_eq!(get_value_from_line(&correct_digits_in_line(l)), 29);
    }
}

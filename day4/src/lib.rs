use std::io::BufRead;

pub fn process_input(input: &Vec<u8>) -> u32 {
    input
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| get_points(&line))
        .sum()
}

// Each line is in the format of: `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53`.
// The numbers before the `|` are the winning numbers and we need to check if there are any winning numbers after `|`.
// Note: There can be multiple spaces between each number.
fn get_points(line: &str) -> u32 {
    let mut iter = line.split(" | ");

    let win_iter = iter.next().unwrap().split(':');

    let winning_numbers: Vec<u32> = win_iter
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let numbers: Vec<u32> = iter
        .next()
        .unwrap()
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let total_winning_nums: u32 = numbers.into_iter().fold(0, |acc, num| {
        if winning_numbers.contains(&num) {
            acc + 1
        } else {
            acc
        }
    });

    if total_winning_nums == 0 {
        0
    } else {
        2_u32.pow(total_winning_nums - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .as_bytes();

        assert_eq!(process_input(&input.to_owned()), 13);
    }

    #[test]
    fn test_get_points() {
        let card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(get_points(card), 8);
    }
}

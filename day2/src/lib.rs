use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Colors {
    Red(u32),
    Green(u32),
    Blue(u32),
}

pub fn process_input(lines: &mut BufReader<File>) -> u32 {
    lines
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| get_value_from_line(&line))
        .sum()
}

fn get_value_from_line(line: &str) -> u32 {
    // Split each line by `:`
    let split: Vec<&str> = line.split(':').collect();

    // Parse the second part of split to get the game combinations by `;`
    // Then get the max value for red, max value for green, max value for blue.
    let valid_sets = split[1]
        .split(';')
        .into_iter()
        .map(|game_combination| {
            let game_combination = game_combination.trim();
            parse_game_combinations(game_combination)
        })
        .fold((0, 0, 0), |acc, comb| {
            if let (Colors::Red(red), Colors::Green(green), Colors::Blue(blue)) = comb {
                (acc.0.max(red), acc.1.max(green), acc.2.max(blue))
            } else {
                acc
            }
        });

    valid_sets.0 * valid_sets.1 * valid_sets.2
}

// Each game combinations will be like `3 blue, 4 red` which
// needs to be parsed into `(Colors::Red(4), Colors::Green(0), Colors::Blue(3))`
fn parse_game_combinations(game_combination: &str) -> (Colors, Colors, Colors) {
    let colors: Vec<&str> = game_combination.split(',').collect();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for color in colors {
        let color = color.trim();
        if color.ends_with("red") {
            red = color
                .split(' ')
                .next()
                .expect("red should have 2 elements")
                .parse::<u32>()
                .expect("red should be a number");
        } else if color.ends_with("green") {
            green = color
                .split(' ')
                .next()
                .expect("green should have 2 elements")
                .parse::<u32>()
                .expect("green should be a number");
        } else if color.ends_with("blue") {
            blue = color
                .split(' ')
                .next()
                .expect("blue should have 2 elements")
                .parse::<u32>()
                .expect("blue should be a number");
        }
    }

    (Colors::Red(red), Colors::Green(green), Colors::Blue(blue))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_by_line() {
        assert_eq!(
            get_value_from_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );

        assert_eq!(
            get_value_from_line(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            630
        );
    }

    #[test]
    fn test_parse_game_combinations() {
        assert_eq!(
            parse_game_combinations("3 blue, 4 red"),
            (Colors::Red(4), Colors::Green(0), Colors::Blue(3)),
        );
        assert_eq!(
            parse_game_combinations("8 green, 6 blue, 20 red"),
            (Colors::Red(20), Colors::Green(8), Colors::Blue(6)),
        );
    }
}

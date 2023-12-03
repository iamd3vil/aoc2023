use std::io::BufRead;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Colors {
    Red(u32),
    Green(u32),
    Blue(u32),
}

pub fn process_input(lines: &Vec<u8>, total_comb: (Colors, Colors, Colors)) -> u32 {
    lines
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| get_value_from_line(&line, &total_comb))
        .sum()
}

fn get_value_from_line(line: &str, total_comb: &(Colors, Colors, Colors)) -> u32 {
    // Split each line by `:`
    let split: Vec<&str> = line.split(':').collect();

    // Split the first part by ` ` to get the game number.
    let game_number: Vec<&str> = split[0].split(' ').collect();
    let game_number = game_number
        .get(1)
        .expect("game number should have 2 elements")
        .parse::<u32>()
        .expect("game number should be a number");

    // Parse the second part of split to get the game combinations by `;`

    let valid_sets = split[1]
        .split(';')
        .into_iter()
        .map(|game_combination| {
            let game_combination = game_combination.trim();
            if let (Colors::Red(red), Colors::Green(green), Colors::Blue(blue)) =
                parse_game_combinations(game_combination)
            {
                if let (Colors::Red(tot_red), Colors::Green(tot_green), Colors::Blue(tot_blue)) =
                    total_comb
                {
                    // Check if the game combination is valid
                    if red > *tot_red || green > *tot_green || blue > *tot_blue {
                        return false;
                    }
                }
            }

            true
        })
        .filter(|b| !*b)
        .count();

    if valid_sets != 0 {
        return 0;
    }

    game_number
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
            get_value_from_line(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                &(Colors::Red(12), Colors::Green(13), Colors::Blue(14)),
            ),
            1
        );

        assert_eq!(
            get_value_from_line(
                "Game 2: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                &(Colors::Red(12), Colors::Green(13), Colors::Blue(14)),
            ),
            0
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

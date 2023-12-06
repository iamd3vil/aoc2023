use std::{collections::BTreeMap, io::BufRead, vec};

#[derive(Clone, Debug, PartialEq)]
enum CardState {
    Scratched,
    NotScratched,
}

#[derive(Clone, Debug, PartialEq)]
struct Card {
    number: u32,
    winning_numbers: u32,
    state: CardState,
}

pub fn process_input(input: &Vec<u8>) -> u32 {
    let all_points: BTreeMap<u32, Card> = input
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| get_points(&line))
        .collect();

    // We have to do multiple passes.
    let mut cur_cards_to_process: Vec<Card> = all_points.values().cloned().collect();

    let mut new_cards: Vec<Card> = vec![];
    let mut has_new_cards = true;

    while has_new_cards {
        for c in cur_cards_to_process.iter_mut() {
            if let CardState::NotScratched = c.state {
                c.state = CardState::Scratched;
                has_new_cards = false;

                if c.winning_numbers != 0 {
                    (c.number + 1..c.number + c.winning_numbers + 1)
                        .into_iter()
                        .for_each(|num| {
                            let wn = all_points.get(&num).unwrap();

                            new_cards.push(Card {
                                number: num,
                                winning_numbers: wn.winning_numbers,
                                state: CardState::NotScratched,
                            });
                        });
                }
            }
        }
        if !new_cards.is_empty() {
            has_new_cards = true;
        }
        cur_cards_to_process.extend_from_slice(&new_cards);
        new_cards.clear();
    }

    cur_cards_to_process.len() as u32
}

// Each line is in the format of: `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53`.
// The numbers before the `|` are the winning numbers and we need to check if there are any winning numbers after `|`.
// Note: There can be multiple spaces between each number.
fn get_points(line: &str) -> (u32, Card) {
    let mut iter = line.split(" | ");

    let mut win_iter = iter.next().unwrap().split(':');

    let card_num = win_iter
        .next()
        .unwrap()
        .split(' ')
        .filter(|num| !num.is_empty())
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let winning_numbers: Vec<u32> = win_iter
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

    (
        card_num,
        Card {
            number: card_num,
            winning_numbers: total_winning_nums,
            state: CardState::NotScratched,
        },
    )
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

        assert_eq!(process_input(&input.to_owned()), 30);
    }

    #[test]
    fn test_get_points() {
        let card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(
            get_points(card),
            (
                1,
                Card {
                    number: 1,
                    winning_numbers: 4,
                    state: CardState::NotScratched
                }
            )
        );
    }
}

use std::collections::BTreeMap;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Debug)]
enum Val {
    Symbol(char),
    Number(char),
}

pub fn process_input(lines: &mut BufReader<File>) -> u32 {
    let mut positions: BTreeMap<(usize, usize), Val> = BTreeMap::new();

    lines
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(x, line)| {
            line.chars().enumerate().for_each(|(y, c)| {
                match c {
                    '.' => return,
                    c if c.is_ascii_digit() => positions.insert((x, y), Val::Number(c)),
                    c => positions.insert((x, y), Val::Symbol(c)),
                };
            });
        });

    let mut numbers: Vec<(Vec<(usize, usize)>, String)> = vec![];

    for ((x, y), val) in positions.iter() {
        // We need to aggregate numbers into the `numbers` vec.
        // `(0, 0)`, `(0, 1)`, `(0, 2)` would be into 1 vector
        // and `(0, 5)`, `(0, 6)` would be another vec.
        if let Val::Number(c) = val {
            let last = numbers.last_mut();
            match last {
                Some(last) => {
                    let last_val = last.0.last().unwrap();
                    let mut last_val_str = last.1.clone();
                    if last_val.0 == *x && last_val.1 + 1 == *y {
                        last.0.push((*x, *y));
                        last_val_str.push(*c);
                        last.1 = last_val_str;
                    } else {
                        numbers.push((vec![(*x, *y)], c.to_string()));
                    }
                }
                None => {
                    numbers.push((vec![(*x, *y)], c.to_string()));
                }
            }
        }
    }

    // We have to check if there is any `*` symbol and if there are 2 part_numbers adjacent to it.
    // If there are 2, we have to multiple both numbers and then sum all numbers.
    let mut gear_positions: BTreeMap<(i32, i32), Vec<&str>> = BTreeMap::new();

    // We have to check if each number has an adjacent symbol either beside it or diagnolly.
    numbers.iter().for_each(|(coords, num)| {
        let positions_to_check: Vec<(i32, i32)> = vec![
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        // let mut valid = false;

        for (x, y) in coords {
            for (x_pos, y_pos) in positions_to_check.iter() {
                let x_pos = *x_pos + *x as i32;
                let y_pos = *y_pos + *y as i32;

                if let Some(Val::Symbol(c)) = positions.get(&(x_pos as usize, y_pos as usize)) {
                    // valid = true;

                    if *c == '*' {
                        gear_positions.entry((x_pos, y_pos)).or_default().push(num);
                    }
                }
            }
        }

        // if valid {
        //     Some(num.parse::<u32>().unwrap())
        // } else {
        //     None
        // }
    });

    gear_positions
        .into_iter()
        // .inspect(|(coords, nums)| println!("{:?} {:?}", coords, nums))
        .map(|(coords, nums)| {
            let uniq_nums = nums.iter().unique().copied().collect::<Vec<&str>>();
            (coords, uniq_nums)
        })
        .filter(|(_coords, nums)| nums.len() == 2)
        .map(|(_coords, nums)| {
            let mul: u32 = nums
                .iter()
                .fold(1, |acc, num| acc * num.parse::<u32>().unwrap());
            mul
        })
        .sum()
}

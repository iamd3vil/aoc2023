use anyhow::Result;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::io::BufRead;
use std::io::Lines;
use std::ops;
use winnow::{
    ascii::digit0,
    combinator::{preceded, separated},
    prelude::*,
};

#[derive(Debug)]
struct Range {
    source: ops::Range<u64>,
    dest: ops::Range<u64>,
}

impl Range {
    fn get_val(&self, v: u64) -> Option<u64> {
        if self.source.contains(&v) {
            Some(self.dest.start + (v - self.source.start))
        } else {
            None
        }
    }
}

pub fn process_input(input: &[u8]) -> Result<u64> {
    let mut iter = input.lines();

    let mut first_line = iter.next().unwrap()?;

    let seeds = get_seeds(&mut first_line).unwrap();

    // Parse seed to soil.
    let seed_to_soil_map = parse_map(&mut iter, 2);
    let soil_to_fert_map = parse_map(&mut iter, 1);
    let fert_to_water_map = parse_map(&mut iter, 1);
    let water_to_light_map = parse_map(&mut iter, 1);
    let light_to_temp_map = parse_map(&mut iter, 1);
    let temp_to_hum_map = parse_map(&mut iter, 1);
    let hum_to_loc_map = parse_map(&mut iter, 1);

    let min_loc = seeds
        .iter()
        .copied()
        .map(|s| {
            let soil = get_value_from_map(&seed_to_soil_map, s);
            let fert = get_value_from_map(&soil_to_fert_map, soil);
            let water = get_value_from_map(&fert_to_water_map, fert);
            let light = get_value_from_map(&water_to_light_map, water);
            let temp = get_value_from_map(&light_to_temp_map, light);
            let hum = get_value_from_map(&temp_to_hum_map, temp);
            get_value_from_map(&hum_to_loc_map, hum)
        })
        .min()
        .unwrap();

    Ok(min_loc)
}

fn get_value_from_map(m: &Vec<Range>, v: u64) -> u64 {
    for r in m {
        match r.get_val(v) {
            Some(rv) => return rv,
            None => continue,
        }
    }
    return v;
}

fn parse_map(iter: &mut Lines<&[u8]>, skip: usize) -> Vec<Range> {
    iter.skip(skip)
        .map(|l| l.unwrap())
        .fold_while(vec![], |mut acc, l| {
            let l = l.trim();
            if l.is_empty() {
                return Done(acc);
            }

            acc.push(parse_range(&l));
            Continue(acc)
        })
        .into_inner()
}

fn parse_range(mut line: &str) -> Range {
    let dig: PResult<Vec<u64>> =
        separated(1.., digit0.parse_to::<u64>(), ' ').parse_next(&mut line);

    let dig = dig.unwrap();

    Range {
        source: dig[1]..dig[1] + dig[2],
        dest: dig[0]..dig[0] + dig[2],
    }
}

pub fn get_seeds(mut line: &str) -> PResult<Vec<u64>> {
    preceded("seeds: ", separated(1.., digit0.parse_to::<u64>(), ' ')).parse_next(&mut line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() -> Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "
        .as_bytes();

        assert_eq!(process_input(&input.to_owned())?, 35);

        Ok(())
    }

    #[test]
    fn test_get_seeds() {
        let mut line = "seeds: 79 14 55 13";

        assert_eq!(get_seeds(&mut line).unwrap(), vec![79, 14, 55, 13]);
    }
}

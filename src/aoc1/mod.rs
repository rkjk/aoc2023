use crate::utils::read_input;
use std::cmp::{max, min};

#[allow(dead_code)]
const NUM_STRINGS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug)]
struct Context {
    full_input: Vec<String>,
}

impl Context {
    #[allow(dead_code)]
    pub fn new(path: &str) -> Self {
        Self {
            full_input: read_input(path).unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn part2(&self) -> u32 {
        let digits = Context::get_digits_arr(&self.full_input);
        let string_digits: Vec<Vec<(usize, u32)>> = self
            .full_input
            .iter()
            .map(|string| {
                let mut tmp = vec![];
                for (ind, s) in NUM_STRINGS.iter().enumerate() {
                    match string.find(s) {
                        Some(v) => tmp.push((v, ind as u32)),
                        None => (),
                    };
                    match string.rfind(s) {
                        Some(v) => tmp.push((v, ind as u32)),
                        None => (),
                    };
                }
                tmp
            })
            .collect();
        digits
            .iter()
            .zip(string_digits.iter())
            .map(|(dig_vec, str_vec)| {
                //println!("dig_vec: {:?}", dig_vec);
                //println!("str_vec: {:?}", str_vec);
                let mut combined_vec: Vec<(usize, u32)> = vec![];
                combined_vec.extend(dig_vec.into_iter());
                combined_vec.extend(str_vec.into_iter());
                combined_vec.sort_by(|a, b| a.0.cmp(&b.0));
                let (res_min, res_max) =
                    (combined_vec[0].1, combined_vec[combined_vec.len() - 1].1);
                let val = res_min * 10 + res_max;
                //println!("val: {}", val);
                //println!();
                val
            })
            .sum()
    }

    fn get_digits_arr(full_input: &Vec<String>) -> Vec<Vec<(usize, u32)>> {
        full_input
            .iter()
            .map(|string| {
                string
                    .chars()
                    .enumerate()
                    .filter_map(|(ind, c)| match c.is_digit(10) {
                        true => Some((ind, c.to_digit(10).unwrap())),
                        false => None,
                    })
                    .collect::<Vec<(usize, u32)>>()
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn part1(&self) -> u32 {
        Context::get_digits_arr(&self.full_input)
            .iter()
            .map(|int_arr| int_arr[0].1 * 10 + int_arr[int_arr.len() - 1].1)
            .sum()
    }
}

#[cfg(test)]
mod aoc1 {
    use super::*;

    #[test]
    fn example1() {
        let context = Context::new("src/aoc1/example");
        println!("Example 1: {}", context.part1());
    }

    #[test]
    fn example2() {
        let context = Context::new("src/aoc1/example2");
        println!("Example 2: {}", context.part2());
    }

    #[test]
    fn part1() {
        let context = Context::new("src/aoc1/input");
        println!("Part1: {}", context.part1());
        println!("Part2: {}", context.part2());
    }
}

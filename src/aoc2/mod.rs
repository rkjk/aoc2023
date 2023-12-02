#[allow(dead_code)]
use crate::utils::read_input;
use std::error::Error;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Unknown,
}

#[derive(Debug)]
struct BallCount {
    pub color: Color,
    pub count: usize,
}

impl BallCount {
    pub fn new() -> Self {
        BallCount {
            color: Color::Unknown,
            count: 0,
        }
    }

    pub fn new_init(color: Color, count: usize) -> Self {
        BallCount {
            color: color,
            count: count,
        }
    }
}

#[derive(Debug)]
struct Set {
    red: BallCount,
    green: BallCount,
    blue: BallCount,
}

impl Set {
    pub fn new(input: Vec<BallCount>) -> Self {
        let mut val = Set {
            red: BallCount::new(),
            green: BallCount::new(),
            blue: BallCount::new(),
        };
        for ball in input.into_iter() {
            match ball.color {
                Color::Red => val.red = ball,
                Color::Green => val.green = ball,
                Color::Blue => val.blue = ball,
                _ => (),
            };
        }
        val
    }

    pub fn check_limits(&self, other: &Set) -> bool {
        let (sr, sg, sb) = (self.red.count, self.green.count, self.blue.count);
        let (or, og, ob) = (other.red.count, other.green.count, other.blue.count);
        sr <= or && sg <= og && sb <= ob
    }
}

#[derive(Debug)]
struct Game {
    pub sets: Vec<Set>,
}

impl Game {
    pub fn new(input: Vec<Set>) -> Game {
        Game { sets: input }
    }
}

struct Context {
    parsed_input: Vec<Game>,
}

impl Context {
    pub fn new(path: &str) -> Self {
        Context {
            parsed_input: Context::parse_input(read_input(path).unwrap()).unwrap(),
        }
    }

    fn parse_ball(input: &str) -> Result<BallCount, &str> {
        let final_split: Vec<&str> = input.trim().split_whitespace().collect();
        let num = final_split[0].parse::<usize>().unwrap();
        let val = match final_split[1] {
            "red" => BallCount::new_init(Color::Red, num),
            "blue" => BallCount::new_init(Color::Blue, num),
            "green" => BallCount::new_init(Color::Green, num),
            _ => BallCount::new(),
        };
        if val.color == Color::Unknown {
            return Err("Unknown ball type. Check input");
        }
        Ok(val)
    }

    fn parse_input<'a>(input: Vec<String>) -> Result<Vec<Game>, &'a str> {
        let vec = input
            .into_iter()
            .map(|val| {
                Game::new(
                    val.split(':')
                        .next_back()
                        .unwrap()
                        .trim()
                        .split(';')
                        .map(|game_str| {
                            Set::new(
                                game_str
                                    .trim()
                                    .split(',')
                                    .map(|ball_plus_count| {
                                        Context::parse_ball(ball_plus_count).unwrap()
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                )
            })
            .collect();
        Ok(vec)
    }

    pub fn part1(&self, limit_set: Set) -> usize {
        let mut res = 0;
        for (ind, game) in self.parsed_input.iter().enumerate() {
            let valid = game
                .sets
                .iter()
                .map(|set| set.check_limits(&limit_set))
                .all(|v| v);
            if valid {
                res += ind + 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod aoc2 {
    use super::*;

    #[test]
    fn example() {
        let context = Context::new("src/aoc2/example");
        println!(
            "Example 1: {}",
            context.part1(Set::new(vec![
                BallCount::new_init(Color::Red, 12),
                BallCount::new_init(Color::Green, 13),
                BallCount::new_init(Color::Blue, 14)
            ]))
        )
    }

    #[test]
    fn actual() {
        let context = Context::new("src/aoc2/input");
        println!(
            "Example 1: {}",
            context.part1(Set::new(vec![
                BallCount::new_init(Color::Red, 12),
                BallCount::new_init(Color::Green, 13),
                BallCount::new_init(Color::Blue, 14)
            ]))
        )
    }
}

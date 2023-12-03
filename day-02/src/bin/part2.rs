use rayon::prelude::*;
use std::u32;

struct Game {
    id: u32,
    red_min: u32,
    blue_min: u32,
    green_min: u32,
    power: u32,
}
impl Game {
    fn new(line: &str) -> Self {
        let mut game = line.split(":");

        let id = game
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        game.last().unwrap().split(";").for_each(|draw| {
            draw.split(",").for_each(|color| {
                let mut it = color.trim_start().trim_end().split(" ");
                let number = it.next().unwrap().parse::<u32>().unwrap();
                let color = it.last().unwrap();
                match color.len() {
                    3 => {
                        if number > red {
                            red = number;
                        }
                    }
                    4 => {
                        if number > blue {
                            blue = number;
                        }
                    }
                    5 => {
                        if number > green {
                            green = number;
                        }
                    }
                    _ => {}
                };
            });
        });

        Game {
            id,
            red_min: red,
            blue_min: blue,
            green_min: green,
            power: red * green * blue,
        }
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let result = part1(input);
    println!("result is: {:?}", result);
}

fn part1(input: &str) -> u32 {
    input.par_lines().map(|line| Game::new(line).power).sum()
}

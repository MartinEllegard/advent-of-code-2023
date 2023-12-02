// #![feature(test)]

use rayon::prelude::*;
use std::u32;

// extern crate test;

// #[bench]
// fn name(b: &mut test::Bencher) {
//     let input = include_str!("input1.txt");
//     b.iter(|| part1(input))
// }
const REDS: u32 = 12;
const GREENS: u32 = 13;
const BLUES: u32 = 14;

fn main() {
    let input = include_str!("input1.txt");
    let result = part1(input);
    println!("result is: {:?}", result);
}

fn part1(input: &str) -> u32 {
    let valid_games: Vec<u32> = input
        .par_lines()
        .filter_map(|line| {
            let mut game = line.split(":");
            let id = game
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let impossible: Vec<bool> = game
                .last()
                .unwrap()
                .split(';')
                .filter_map(|game| {
                    let takes = game.split(",");
                    // returns some if one of the inputs is too high
                    let any_invalid: Vec<bool> = takes
                        .filter_map(|take| {
                            let mut it = take.trim_start().trim_end().split(" ");
                            let number =
                                it.next().unwrap().parse::<u32>().expect("must be a number");
                            let color = it.last().expect("color must exist");
                            // println!("color: {0}, number: {1}", &color, &number);
                            let ok = match color.len() {
                                3 => number <= REDS,
                                4 => number <= BLUES,
                                5 => number <= GREENS,
                                _ => false,
                            };
                            match ok {
                                true => None,
                                false => Some(true),
                            }
                        })
                        .collect();
                    if any_invalid.len() > 0 {
                        return Some(true);
                    } else {
                        return None;
                    }
                })
                .collect();

            if impossible.len() > 0 {
                return None;
            } else {
                return Some(id);
            }
        })
        .collect::<Vec<u32>>();
    valid_games.iter().sum()
}

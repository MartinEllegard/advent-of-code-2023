#![feature(test)]

use lazy_static::lazy_static;
use regex::Regex;
use std::u32;
use strum::{IntoEnumIterator, VariantNames};
use strum_macros::{EnumString, EnumVariantNames};
extern crate test;

#[derive(EnumString, Debug, EnumVariantNames)]
enum Digits {
    #[strum(serialize = "one", serialize = "1")]
    One,
    #[strum(serialize = "two", serialize = "2")]
    Two,
    #[strum(serialize = "three", serialize = "3")]
    Three,
    #[strum(serialize = "four", serialize = "4")]
    Four,
    #[strum(serialize = "five", serialize = "5")]
    Five,
    #[strum(serialize = "six", serialize = "6")]
    Six,
    #[strum(serialize = "seven", serialize = "7")]
    Seven,
    #[strum(serialize = "eight", serialize = "8")]
    Eight,
    #[strum(serialize = "nine", serialize = "9")]
    Nine,
}

lazy_static! {
    static ref FIRST_DIGIT: Regex = {
        let digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "[1-9]",
        ];
        let expression = digits.join("|");
        Regex::new(expression.as_str()).unwrap()
    };
}

#[bench]
fn name(b: &mut test::Bencher) {
    let input = include_str!("input1.txt");
    b.iter(|| part1(input))
}

fn main() {
    let input = include_str!("input1.txt");
    let result = part1(input);
    println!("result is: {:?}", result);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut output = Vec::<u32>::with_capacity(lines.clone().count());
    for line in lines {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        match digits.len() {
            0 => {
                continue;
            }
            1 => output.push(*digits.first().unwrap()),
            _ => {
                let out = [digits.first().unwrap(), digits.last().unwrap()]
                    .iter()
                    .fold("".to_string(), |acc, x| acc + &x.to_string());
                output.push(out.parse::<u32>().unwrap());
            }
        }
    }
    output.iter().sum()
}

#![feature(test)]

use std::u32;

extern crate test;

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
    input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let first = digits.first().expect("must contain a digit");
            let out = match digits.last() {
                Some(last) => format!("{0}{1}", first, last),
                None => format!("{0}{1}", first, first),
            };
            out.parse::<u32>().expect("Must be a number")
        })
        .sum()
}

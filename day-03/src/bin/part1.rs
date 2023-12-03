#![feature(test)]

use rayon::prelude::*;
use std::{
    collections::{btree_map::Values, BTreeMap},
    u32,
};

extern crate test;

#[bench]
fn name(b: &mut test::Bencher) {
    let input = include_str!("input1.txt");
    b.iter(|| process(input))
}

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("result is: {:?}", result);
}

enum Schema {
    Empty,
    Number(u32),
    Symbol(char),
}

fn process(input: &str) -> u32 {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| {
                (
                    (y, x),
                    match char {
                        '.' => Schema::Empty,
                        c if c.is_ascii_digit() => Schema::Number(c.to_digit(10).unwrap()),
                        _ => Schema::Symbol(char),
                    },
                )
            })
        })
        .collect::<BTreeMap<(usize, usize), Schema>>();

    let mut numbers: Vec<Vec<((usize, usize), u32)>> = vec![];
    for ((x, y), value) in map.iter() {
        if let Schema::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(ln) => todo!(),
                        None => todo!(),
                    }
                }
                None => numbers.push(vec![((*x, *y), *num)]),
            }
        }
    }
    32 as u32
}

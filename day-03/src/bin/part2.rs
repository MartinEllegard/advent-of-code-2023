#![feature(test)]

use itertools::Itertools;
// use rayon::prelude::*;
use std::{collections::BTreeMap, u32};

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

enum Value {
    Empty,
    Number(u32),
    Symbol(char),
}

fn process(input: &str) -> String {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| {
                (
                    (y as i32, x as i32),
                    match char {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => Value::Number(c.to_digit(10).unwrap()),
                        _ => Value::Symbol(char),
                    },
                )
            })
        })
        .collect::<BTreeMap<(i32, i32), Value>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers.iter_mut().last().unwrap();
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![((*x, *y), *num)])
                            }
                        }
                        None => todo!(),
                    }
                }
                None => numbers.push(vec![((*x, *y), *num)]),
            }
        }
    }

    let mut total = 0;

    for symbol in map
        .iter()
        .filter(|(key, value)| matches!(value, Value::Symbol('*')))
    {
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let positions_to_check: Vec<(i32, i32)> = positions
            .iter()
            .map(|outer_position| {
                (
                    outer_position.0 + symbol.0 .1,
                    outer_position.1 + symbol.0 .0,
                )
            })
            .collect();

        let mut indexes_of_numbers = vec![];

        for position in positions_to_check {
            for (i, num_list) in numbers.iter().enumerate() {
                if num_list
                    .iter()
                    .find(|(num_position, _)| num_position == &position)
                    .is_some()
                {
                    indexes_of_numbers.push(i);
                }
            }
        }

        let is_gear = indexes_of_numbers.iter().unique().count() == 2;

        if is_gear {
            total += indexes_of_numbers
                .iter()
                .unique()
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .product::<usize>();
        }
    }

    total.to_string()
}

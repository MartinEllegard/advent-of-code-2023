use std::collections::BTreeMap;

use crate::custom_error::AocError;
use itertools::Itertools;

enum Value {
    Empty,
    Number(u32),
    Symbol(char),
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
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

    for num_list in numbers {
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

        let num_positions: Vec<(i32, i32)> = num_list.iter().map(|((y, x), _)| (*x, *y)).collect();

        let positions_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions
                    .iter()
                    .map(|outer_position| (outer_position.0 + pos.1, outer_position.1 + pos.0))
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();

        let is_part_number = positions_to_check.iter().any(|pos| {
            let value = map.get(pos);
            if let Some(Value::Symbol(_)) = value {
                true
            } else {
                false
            }
        });

        if is_part_number {
            total += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        }
    }

    Ok(total.to_string())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_process() -> miette::Result<()> {
//         todo!("haven't built test yet");
//         let input = ;
//         assert_eq!("", process(input)?);
//         Ok(())
//     }
// }

use crate::custom_error::AocError;
use rayon::prelude::*;

const REDS: u32 = 12;
const GREENS: u32 = 13;
const BLUES: u32 = 14;

pub fn process(input: &str) -> miette::Result<String, AocError> {
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
    Ok(valid_games.iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = r"
        
        ";
        assert_eq!("", process(input)?);
        Ok(())
    }
}

use crate::custom_error::AocError;
use rayon::prelude::*;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .par_lines()
        .map(|line| power(line))
        .sum::<u32>()
        .to_string())
}

fn power(line: &str) -> u32 {
    let game = line.split(":");

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

    return red * green * blue;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_process() -> miette::Result<()> {
//         todo!("haven't built test yet");
//         let input = "";
//         assert_eq!("", process(input)?);
//         Ok(())
//     }
// }

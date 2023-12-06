use crate::custom_error::AocError;
use rayon::prelude::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .par_lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let first = digits.first().expect("must contain a digit");
            let out = match digits.last() {
                Some(last) => format!("{0}{1}", first, last),
                None => format!("{0}{1}", first, first),
            };
            out.parse::<u32>().expect("Must be a number")
        })
        .sum::<u32>()
        .to_string())
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

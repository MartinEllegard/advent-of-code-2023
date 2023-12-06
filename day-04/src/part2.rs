use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        Ok(())
        // todo!("haven't built test yet");
        // let input = "";
        // assert_eq!("", process(input)?);
        // Ok(())
    }
}

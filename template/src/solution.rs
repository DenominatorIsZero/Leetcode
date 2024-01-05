use crate::custom_error::LCError;
use rstest::*;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, LCError> {
    todo!("day01");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}

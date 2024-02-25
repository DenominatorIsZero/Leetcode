use crate::custom_error::LCError;

#[tracing::instrument]
pub fn process(input: i32) -> miette::Result<bool, LCError> {
    if input < 0 {
        return Ok(false);
    }
    let mut original = input;
    let mut reversed = 0;

    while original > 0 {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }
    Ok(input == reversed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(121, true)]
    #[case(-121, false)]
    #[case(10, false)]
    #[tracing::instrument]
    fn test_process(#[case] input: i32, #[case] expected: bool) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}

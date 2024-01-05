use crate::custom_error::LCError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, LCError> {
    let mut out: f64 = 2.0;
    for _ in 0..1000 {
        out = out.powf(1.2);
    }
    Ok("Test".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("Test".to_string(), process(input)?);
        Ok(())
    }
}

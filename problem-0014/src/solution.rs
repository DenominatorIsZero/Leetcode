use std::str::Chars;

use crate::custom_error::LCError;

#[tracing::instrument]
pub fn process(strs: Vec<String>) -> miette::Result<String, LCError> {
    let mut chars: Vec<Chars> = strs.iter().map(|string| string.chars()).collect();

    let mut prefix: Vec<char> = Vec::new();
    loop {
        if let Some(this_char) = chars[0].next() {
            for other in chars.iter_mut().skip(1) {
                let other_char = other.next();
                if other_char.is_none() || other_char.unwrap() != this_char {
                    return Ok(prefix.into_iter().collect());
                }
            }
            prefix.push(this_char);
        } else {
            return Ok(prefix.into_iter().collect());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()], "fl".to_string())]
    #[case(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()], "".to_string())]
    #[tracing::instrument]
    fn test_process(#[case] input: Vec<String>, #[case] expected: String) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}

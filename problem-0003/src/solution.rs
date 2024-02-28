use std::collections::HashMap;

use crate::custom_error::LCError;

#[tracing::instrument]
pub fn process(input: String) -> miette::Result<i32, LCError> {
    let mut start_index = -1;
    let mut record = 0;
    let mut map: HashMap<char, i32> = HashMap::new();
    for (index, char) in input.chars().enumerate() {
        // if the char is already in the map, we might have to update the start_index
        if let Some(seen_index) = map.insert(char, index as i32) {
            // only update the start_index if the duplicate in the current
            // substring we are looking at
            start_index = start_index.max(seen_index);
        }
        record = record.max(index as i32 - start_index);
    }
    Ok(record)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("abcabcbb".to_string(), 3)]
    #[case("bbbb".to_string(), 1)]
    #[case("pwwkew".to_string(), 3)]
    #[case("".to_string(), 0)]
    #[case(" ".to_string(), 1)]
    #[case("au".to_string(), 2)]
    #[tracing::instrument]
    fn test_process(#[case] input: String, #[case] expected: i32) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}

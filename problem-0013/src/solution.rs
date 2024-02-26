use crate::custom_error::LCError;

#[tracing::instrument]
pub fn process(roman: String) -> miette::Result<i32, LCError> {
    let digits: Vec<i32> = roman
        .chars()
        .map(|v| match v {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Unknown character."),
        })
        .collect();
    let mut value = 0;

    let mut index = 0;
    while index < digits.len() - 1 {
        let current = digits[index];
        let next = digits[index + 1];
        if current < next {
            value += next - current;
            index += 2;
        } else {
            value += current;
            index += 1;
        }
    }
    if index < digits.len() {
        value += digits[index];
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("III", 3)]
    #[case("LVIII", 58)]
    #[case("MCMXCIV", 1994)]
    #[tracing::instrument]
    fn test_process(#[case] roman: String, #[case] expected: i32) -> miette::Result<()> {
        assert_eq!(expected, process(roman)?);
        Ok(())
    }
}

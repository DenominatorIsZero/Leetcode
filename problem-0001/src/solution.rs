use crate::custom_error::LCError;

#[tracing::instrument]
pub fn process(nums: Vec<i32>, target: i32) -> miette::Result<Vec<i32>, LCError> {
    for first_index in 0..nums.len() {
        for second_index in first_index + 1..nums.len() {
            if nums[first_index] + nums[second_index] == target {
                return Ok(vec![first_index as i32, second_index as i32]);
            }
        }
    }
    Err(LCError::IoError(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        format!("No solution found for 'nums' = '{nums:?}, 'target' = '{target}'"),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case([2, 7, 11, 15].to_vec(), 9, [0, 1].to_vec())]
    #[case([3,2,4].to_vec(), 6, [1, 2].to_vec())]
    #[case([3,3].to_vec(), 6, [0, 1].to_vec())]
    #[tracing::instrument]
    fn test_process(
        #[case] nums: Vec<i32>,
        #[case] target: i32,
        #[case] expected: Vec<i32>,
    ) -> miette::Result<()> {
        let nums = nums.to_vec();
        let expexted = expected.to_vec();
        assert_eq!(expexted, process(nums, target)?);
        Ok(())
    }
}

use crate::custom_error::LCError;
use std::collections::HashMap;

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

#[tracing::instrument]
pub fn process_2(nums: Vec<i32>, target: i32) -> miette::Result<Vec<i32>, LCError> {
    let summands: HashMap<i32, usize> = nums
        .iter()
        .enumerate()
        .map(|(i, num)| (num - target, i))
        .collect();

    let (index_1, index_2) = nums
        .iter()
        .enumerate()
        .filter_map(|(index_1, num)| summands.get(&(-num)).map(|index_2| (index_1, *index_2)))
        .find(|(index_1, index_2)| index_1 != index_2)
        .unwrap();

    Ok(vec![index_1 as i32, index_2 as i32])
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
    #[rstest]
    #[case([2, 7, 11, 15].to_vec(), 9, [0, 1].to_vec())]
    #[case([3,2,4].to_vec(), 6, [1, 2].to_vec())]
    #[case([3,3].to_vec(), 6, [0, 1].to_vec())]
    #[tracing::instrument]
    fn test_process_2(
        #[case] nums: Vec<i32>,
        #[case] target: i32,
        #[case] expected: Vec<i32>,
    ) -> miette::Result<()> {
        let nums = nums.to_vec();
        let expexted = expected.to_vec();
        assert_eq!(expexted, process_2(nums, target)?);
        Ok(())
    }
}

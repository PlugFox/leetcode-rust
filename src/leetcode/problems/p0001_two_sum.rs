use crate::Solution;

/*
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * You can return the answer in any order.
 *
 * Example 1:
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
 *
 * Example 2:
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 *
 * Example 3:
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 *
 *
 * Constraints:
 *
 * 2 <= nums.length <= 104
 * -109 <= nums[i] <= 109
 * -109 <= target <= 109
 * Only one valid answer exists.
 *
 *
 * Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
 */
impl Solution {
    // Add implementations of LeetCode problems here
    // Example for Two Sum (problem 1):
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            let a = nums[i];
            for j in (i + 1)..nums.len() {
                let b = nums[j];
                if a + b == target {
                    return vec![i.try_into().unwrap(), j.try_into().unwrap()];
                }
            }
        }
        vec![-1, -1] // If no solution is found, return an invalid result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;

    #[derive(Deserialize)]
    struct TestCase {
        input: Input,
        output: Vec<i32>,
    }

    #[derive(Deserialize)]
    struct Input {
        nums: Vec<i32>,
        target: i32,
    }

    #[test]
    fn test_two_sum() {
        let data = fs::read_to_string("data/leetcode/p0001_two_sum.json")
            .expect("Unable to read test file");
        let test_cases: Vec<TestCase> = serde_json::from_str(&data).expect("Invalid JSON format");

        for case in test_cases {
            assert_eq!(
                Solution::two_sum(case.input.nums, case.input.target),
                case.output
            );
        }
    }
}

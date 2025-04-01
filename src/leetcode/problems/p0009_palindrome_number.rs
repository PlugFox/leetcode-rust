use crate::Solution;

/*
 * Given an integer x, return true if x is a palindrome, and false otherwise.
 *
 * Example 1:
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 *
 * Example 2:
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
 *
 * Example 3:
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 *
 * Constraints:
 * -231 <= x <= 231 - 1
 *
 * Follow up: Could you solve it without converting the integer to a string?
 */
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s = x.to_string();
        s.chars().eq(s.chars().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;

    #[derive(Deserialize)]
    struct TestCase {
        input: i32,
        output: bool,
    }

    #[test]
    fn test_is_palindrome() {
        let data = fs::read_to_string("data/leetcode/p0009_palindrome_number.json")
            .expect("Unable to read test file");
        let test_cases: Vec<TestCase> = serde_json::from_str(&data).expect("Invalid JSON format");

        for case in test_cases {
            assert_eq!(Solution::is_palindrome(case.input), case.output);
        }
    }
}

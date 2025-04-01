use rustleet::Solution;
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
    let data =
        fs::read_to_string("data/leetcode/p0001_two_sum.json").expect("Unable to read test file");
    let test_cases: Vec<TestCase> = serde_json::from_str(&data).expect("Invalid JSON format");

    for case in test_cases {
        assert_eq!(
            Solution::two_sum(case.input.nums, case.input.target),
            case.output
        );
    }
}

use rustleet::Solution;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: leetcode <problem_number>");
        return;
    }

    let problem_number = &args[1];

    match problem_number.as_str() {
        "1" => {
            println!("Running Two Sum solution...");
            let nums = vec![2, 7, 11, 15];
            let target = 9;
            let result = Solution::two_sum(nums, target);
            println!("Result: {:?}", result);
        }
        "9" => {
            println!("Running Palindrome Number solution...");
            let x = 121;
            let result = Solution::is_palindrome(x);
            println!("Result: {:?}", result);
        }
        _ => println!("Problem {} not implemented yet.", problem_number),
    }
}

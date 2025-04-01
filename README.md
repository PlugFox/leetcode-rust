# 🦀 Rust Leetcode

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Rust Book](https://img.shields.io/badge/book-Rust%20Book-blue.svg)](https://doc.rust-lang.org/book/)
[![Alt Rust Book](https://img.shields.io/badge/book-Improved%20Rust%20Book-blue.svg)](https://rust-book.cs.brown.edu/)
[![Rustlings](https://img.shields.io/badge/rustlings-Exercises-orange.svg)](https://github.com/rust-lang/rustlings)
[![Leetcode](https://img.shields.io/badge/leetcode-Questions-blue.svg)](https://leetcode.com/problemset/all/)
[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://opensource.org/licenses/MIT)


A collection of LeetCode problem solutions implemented in Rust. This repository serves as a learning resource for Rust programming language while solving algorithmic challenges.

## 📋 Repository Structure

```
leetcode-rust/
├── bin/                   # Executable Rust files
│   ├── guessing_game.rs   # Sample guessing game
│   ├── hello.rs           # Hello world example
│   └── leetcode.rs        # Main LeetCode runner
├── data/                  # Test data for LeetCode problems
│   └── leetcode/
│       ├── p0001_two_sum.json
│       └── p0009_palindrome_number.json
├── src/                   # Source code directory
│   ├── lib.rs             # Library entry point
│   └── leetcode/          # LeetCode solutions
│       ├── problems.rs    # Problem modules registry
│       └── problems/      # Individual problem solutions
│           ├── p0001_two_sum.rs
│           └── p0009_palindrome_number.rs
└── Cargo.toml             # Rust package configuration
```

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (comes with Rust)

### Building the Project

```bash
# Build the project
cargo build

# Build with optimizations for release
cargo build --release
# or
make build
```

## 🧩 Working with LeetCode Problems

### Directory Structure for Solutions

Each LeetCode problem follows this pattern:
- Solution code: `src/leetcode/problems/p{number}_{name}.rs`
- Test data: `data/leetcode/p{number}_{name}.json`

### Adding a New LeetCode Problem

1. Create a new file in `src/leetcode/problems` following the naming convention:
   ```
   p{problem_number}_{problem_name_snake_case}.rs
   ```

2. Implement your solution in the new file:
   ```rust
   pub struct Solution;

   impl Solution {
       pub fn your_method_name(/* parameters */) -> /* return type */ {
           // Your solution here
       }
   }

   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_your_method_name() {
           // Test cases
           assert_eq!(Solution::your_method_name(/* test input */), /* expected output */);
       }
   }
   ```

3. Register your solution in `src/leetcode/problems.rs`:
   ```rust
   pub mod p{problem_number}_{problem_name};
   ```

4. Optionally, add test data in `data/leetcode/p{problem_number}_{problem_name}.json`

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific problem
cargo test p0001_two_sum

# Run tests with detailed output
cargo test -- --nocapture
```

### Running the LeetCode Solutions

```bash
# Run the main LeetCode runner
cargo run --bin leetcode <problem_number>

# Run a specific example
cargo run --bin hello
cargo run --bin guessing_game
```

## 🧪 Development Workflow

### Available Make Commands

```bash
# Build the complete pipeline (generate, format, check, test)
make all

# Format code
make format

# Clean project (remove all build artifacts)
make clean

# Run tests
make test

# Update dependencies
make update

# Check for outdated dependencies
make outdated

# Get help on available commands
make help
```

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-solution`)
3. Commit your changes (`git commit -m 'Add some amazing solution'`)
4. Push to the branch (`git push origin feature/amazing-solution`)
5. Open a Pull Request
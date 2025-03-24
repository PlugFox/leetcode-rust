// Hello World program in Rust
// How to run:
//
// $ cargo run --bin hello
//
// $ rustc -O hello.rs -o hello.run && ./hello.run && rm hello.run
//
// $ rustc -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort \
//      -C strip=symbols hello.rs -o hello.run && ./hello.run && rm hello.run
fn main() {
    println!("Hello, LeetCode!");
}

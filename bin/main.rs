// cargo run
// rustc -O main.rs -o main.run && ./main.run && rm main.run
// rustc -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort \
//  -C strip=symbols main.rs -o main.run && ./main.run && rm main.run
fn main() {
    println!("Hello, LeetCode!");
}

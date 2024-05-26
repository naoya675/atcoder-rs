use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    println!("{}", "x".repeat(s.len()));
}

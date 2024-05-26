use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let res = n % 2 == 0 && s[..n / 2] == s[n / 2..];
    println!("{}", if res { "Yes" } else { "No" });
}

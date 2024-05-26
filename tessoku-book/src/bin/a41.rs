use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut res = false;
    for i in 2..n {
        if s[i - 2] == s[i - 1] && s[i - 1] == s[i] {
            res = true;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}

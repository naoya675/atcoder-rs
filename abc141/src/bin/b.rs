use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut res = true;
    for i in 0..s.len() {
        if i % 2 == 0 && s[i] == 'L' {
            res = false;
        }
        if i % 2 == 1 && s[i] == 'R' {
            res = false;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let res = s.iter().all(|&s| s == 'A') | s.iter().all(|&s| s == 'B');
    println!("{}", if !res { "Yes" } else { "No" });
}

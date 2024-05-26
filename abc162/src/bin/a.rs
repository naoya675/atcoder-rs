use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let res = n.iter().any(|&n| n == '7');
    println!("{}", if res { "Yes" } else { "No" });
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut res = 0;
    for i in 0..n {
        if s[i] == '1' {
            res += 1;
        }
    }
    println!("{}", if res % 2 == k % 2 { "Yes" } else { "No" });
}

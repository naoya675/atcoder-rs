use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut res = n - 1;
    for s in s.windows(2) {
        if s[0] != s[1] {
            res -= 1;
        }
    }
    println!("{}", (res + k * 2).min(n - 1));
}

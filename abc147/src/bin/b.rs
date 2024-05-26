use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut res = 0;
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            res += 1;
        }
    }
    println!("{}", res);
}

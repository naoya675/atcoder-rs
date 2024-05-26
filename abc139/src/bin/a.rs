use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut res = 0;
    for i in 0..3 {
        if s[i] == t[i] {
            res += 1;
        }
    }
    println!("{}", res);
}

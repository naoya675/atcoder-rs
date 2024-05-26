use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut res = t.len();
    for s in s.windows(t.len()) {
        let mut change = 0;
        for i in 0..s.len() {
            if s[i] != t[i] {
                change += 1;
            }
        }
        res = res.min(change);
    }
    println!("{}", res);
}

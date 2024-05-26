use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    println!("{}", s.iter().zip(t.iter()).filter(|(s, t)| s != t).count());
}

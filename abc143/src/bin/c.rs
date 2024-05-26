use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        mut s: Chars,
    }
    s.dedup();
    println!("{}", s.len());
}

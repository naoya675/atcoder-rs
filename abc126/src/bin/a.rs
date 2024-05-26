use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        k: usize,
        mut s: Chars,
    }
    s[k - 1] = s[k - 1].to_ascii_lowercase();
    println!("{}", s.iter().collect::<String>());
}

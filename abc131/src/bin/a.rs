use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    if s[0] == s[1] || s[1] == s[2] || s[2] == s[3] {
        println!("Bad");
    } else {
        println!("Good");
    }
}

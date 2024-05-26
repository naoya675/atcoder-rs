use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    if s[2] == s[3] && s[4] == s[5] {
        println!("Yes");
    } else {
        println!("No");
    }
}

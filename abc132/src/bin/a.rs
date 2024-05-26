use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s.sort();
    if s[0] == s[1] && s[1] != s[2] && s[2] == s[3] {
        println!("Yes");
    } else {
        println!("No");
    }
}

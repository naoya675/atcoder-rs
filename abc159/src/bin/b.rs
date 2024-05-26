use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let res = is_palindrome(&s)
        && is_palindrome(&s[..(s.len() - 1) / 2])
        && is_palindrome(&s[(s.len() + 1) / 2..]);
    println!("{}", if res { "Yes" } else { "No" });
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

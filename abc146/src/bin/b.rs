use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: u8,
        s: Chars,
    }
    let s = s
        .iter()
        .map(|&s| ('A' as u8 + (n + s as u8 - 'A' as u8) % 26) as char)
        .collect::<String>();
    println!("{}", s)
}

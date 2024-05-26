use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let res = n
        .iter()
        .map(|n| n.to_digit(10).unwrap() as usize)
        .sum::<usize>();
    println!("{}", if res % 9 == 0 { "Yes" } else { "No" });
}

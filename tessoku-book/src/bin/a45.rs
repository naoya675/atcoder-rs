use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        c: char,
        a: Chars,
    }
    let c = match c {
        'R' => 2,
        'B' => 1,
        'W' => 0,
        _ => unreachable!(),
    };
    let a = a.into_iter().map(|a| match a {
        'R' => 2,
        'B' => 1,
        'W' => 0,
        _ => unreachable!(),
    });
    println!("{}", if a.sum::<i32>() % 3 == c { "Yes" } else { "No" });
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n],
    }
    let res = d.windows(3).any(|d| d.iter().all(|(d1, d2)| d1 == d2));
    println!("{}", if res { "Yes" } else { "No" });
}

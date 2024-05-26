use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        k: i64,
    }
    println!("{} {}", (0 - (k - a)).max(0), (b - (k - a).max(0)).max(0));
}

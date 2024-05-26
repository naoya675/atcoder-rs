use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }
    println!("{}", (n % k).min(k - n % k));
}

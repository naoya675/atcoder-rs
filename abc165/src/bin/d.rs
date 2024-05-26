use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        n: i64,
    }
    println!("{}", floor(a, b, (b - 1).min(n)));
}

fn floor(a: i64, b: i64, x: i64) -> i64 {
    a * x / b - a * (x / b)
}

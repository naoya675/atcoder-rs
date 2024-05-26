use proconio::input;

fn main() {
    input! {
        x: i64,
        k: i64,
        d: i64,
    }
    let x = x.abs();
    if (k - k.min(x / d)) % 2 == 0 {
        println!("{}", (x - d * k.min(x / d)).abs());
    } else {
        println!("{}", (x - d * k.min(x / d) - d).abs());
    }
}

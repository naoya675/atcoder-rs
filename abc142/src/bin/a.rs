use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    println!("{}", (n - n / 2) as f64 / n as f64);
}

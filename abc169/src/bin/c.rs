use proconio::input;

fn main() {
    input! {
        a: i64,
        b: f64,
    }
    println!("{}", (a * (b * 100.).round() as i64) / 100);
}

use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    }
    println!("{}", (n / (a + b)) * a + (n % (a + b)).min(a));
}

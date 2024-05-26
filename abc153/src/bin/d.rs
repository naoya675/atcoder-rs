use proconio::input;

fn main() {
    input! {
        h: i64,
    }
    println!("{}", f(h));
}

fn f(n: i64) -> i64 {
    if n == 1 {
        return 1;
    }
    2 * f(n / 2) + 1
}

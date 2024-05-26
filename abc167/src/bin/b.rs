use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        _: i32,
        k: i32,
    }
    if k <= a + b {
        println!("{}", a.min(k));
    } else {
        println!("{}", a - (k - a - b));
    }
}

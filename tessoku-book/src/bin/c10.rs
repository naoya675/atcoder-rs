use proconio::input;

fn main() {
    input! {
        w: i64,
    }
    let m = 1_000_000_007;
    let res = 12 * power(7, w - 1, m) % m;
    println!("{}", res);
}

fn power(a: i64, b: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut a = a;
    let mut b = b;
    while b > 0 {
        if b % 2 == 1 {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b >>= 1;
    }
    res
}

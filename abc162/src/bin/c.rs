use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        k: i64,
    }
    let mut res = 0;
    for (i, j, k) in iproduct!(1..=k, 1..=k, 1..=k) {
        res += gcd(gcd(i, j), k);
    }
    println!("{}", res);
}

fn gcd(a: i64, b: i64) -> i64 {
    assert!(a > 0);
    assert!(b > 0);
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i64,
        a: [i64; n],
    }
    let m = 1_000_000_007;
    let mut map = HashMap::new();
    let mut res = 0_usize;
    for i in 0..n {
        if a[i] % m == 0 {
            res += i;
        } else if let Some(v) = map.get(&division(p, a[i] % m, m)) {
            res += v;
        }
        *map.entry(a[i] % m).or_default() += 1;
    }
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

fn division(a: i64, b: i64, m: i64) -> i64 {
    (a * power(b, m - 2, m)) % m
}

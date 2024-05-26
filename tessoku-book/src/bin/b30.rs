use proconio::input;

fn main() {
    input! {
        h: i64,
        w: i64,
    }
    let m = 1_000_000_007;
    let mut a = 1;
    let mut b = 1;
    for i in 1..=h + w - 2 {
        a = (a * i) % m;
    }
    for i in 1..=h - 1 {
        b = (b * i) % m;
    }
    for i in 1..=w - 1 {
        b = (b * i) % m;
    }
    println!("{}", division(a, b, m))
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

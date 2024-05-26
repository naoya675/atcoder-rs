use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let modulus: i64 = 1_000_000_007;
    let res = (power(10, n, modulus) - power(9, n, modulus) - power(9, n, modulus)
        + power(8, n, modulus)
        + modulus
        + modulus)
        % modulus;
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

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let modulus: i64 = 1_000_000_007;
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = (acc[i] + a[i]) % modulus;
    }
    let mut res = 0;
    for i in 0..n {
        res = (res + a[i] * (modulus + acc[n] - acc[i + 1])) % modulus;
    }
    println!("{}", res);
}

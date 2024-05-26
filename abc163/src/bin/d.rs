use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }
    let modulus: i64 = 1_000_000_007;
    let mut res = 0;
    for i in k..=n + 1 {
        res = (res + (n + 1 - i) * i + 1) % modulus;
    }
    println!("{}", res);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let modulus: i64 = 1_000_000_007;
    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        let mut acc = vec![0; k + 2];
        for j in 0..=k {
            acc[j + 1] = (acc[j] + dp[i][j]) % modulus;
        }
        for j in 0..=k {
            dp[i + 1][j] = (acc[j + 1] - acc[j - j.min(a[i])] + modulus) % modulus;
        }
    }
    println!("{}", dp[n][k]);
}

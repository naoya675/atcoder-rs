use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [usize; n],
        t: [usize; m],
    }
    let modulus: i64 = 1_000_000_007;
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = (dp[i][j] + dp[i - 1][j - 1] + 1) % modulus;
            }
            dp[i][j] = (dp[i][j] + dp[i - 1][j]) % modulus;
            dp[i][j] = (dp[i][j] + dp[i][j - 1]) % modulus;
            dp[i][j] = (dp[i][j] + modulus - dp[i - 1][j - 1]) % modulus;
        }
    }
    println!("{}", (dp[n][m] + 1) % modulus);
}

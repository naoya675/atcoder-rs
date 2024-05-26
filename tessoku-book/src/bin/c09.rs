use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = vec![0; n + 1];
    for i in 0..n {
        dp[i + 1] = dp[i + 1].max(dp[i]);
        if i + 2 <= n {
            dp[i + 2] = dp[i + 2].max(dp[i] + a[i]);
        }
    }
    println!("{}", dp[n]);
}

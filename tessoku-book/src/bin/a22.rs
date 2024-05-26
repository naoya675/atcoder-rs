use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 1],
    }
    let mut dp = vec![-1_000_000_000; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[a[i] - 1] = dp[a[i] - 1].max(dp[i] + 100);
        dp[b[i] - 1] = dp[b[i] - 1].max(dp[i] + 150);
    }
    println!("{}", dp[n - 1]);
}

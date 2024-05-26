use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }
    let mut dp = vec![vec![0.; n + 1]; n + 1];
    dp[0][0] = 1.;
    for i in 0..n {
        for j in 0..n {
            dp[i + 1][j + 1] += dp[i][j] * p[i];
            dp[i + 1][j] += dp[i][j] * (1. - p[i]);
        }
    }
    println!("{}", dp[n][n / 2 + 1..].iter().sum::<f64>());
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[0][i] = a[i];
    }
    for i in 0..n {
        for j in i + 1..n {
            if i % 2 == n % 2 {
                dp[i + 1][j] = dp[i][j].max(dp[i][j - 1]);
            } else {
                dp[i + 1][j] = dp[i][j].min(dp[i][j - 1]);
            }
        }
    }
    println!("{}", dp[n - 1][n - 1]);
}

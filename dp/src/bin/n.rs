use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut dp = vec![vec![1 << 60; n + 1]; n + 1];
    for i in 0..n {
        dp[i][i + 1] = 0;
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len;
            for k in i + 1..j {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + acc[j] - acc[i]);
            }
        }
    }
    println!("{}", dp[0][n]);
}

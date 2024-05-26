use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for len in 1..=n {
        for i in 0..=n - len {
            let j = i + len;
            if (n - len) % 2 == 0 {
                dp[i][j] = (dp[i + 1][j] + a[i]).max(dp[i][j - 1] + a[j - 1]);
            } else {
                dp[i][j] = (dp[i + 1][j] - a[i]).min(dp[i][j - 1] - a[j - 1]);
            }
        }
    }
    println!("{}", dp[0][n]);
}

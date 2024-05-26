use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for i in 1..n {
        if s[i - 1] == s[i] {
            dp[i - 1][i] = 2;
        } else {
            dp[i - 1][i] = 1;
        }
    }
    for k in 2..n {
        for i in 0..n - k {
            let j = i + k;
            dp[i][j] = dp[i][j].max(dp[i + 1][j]);
            dp[i][j] = dp[i][j].max(dp[i][j - 1]);
            if s[i] == s[j] {
                dp[i][j] = dp[i][j].max(dp[i + 1][j - 1] + 2);
            }
        }
    }
    println!("{}", dp[0][n - 1]);
}

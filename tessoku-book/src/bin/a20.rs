use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let slen = s.len();
    let tlen = t.len();
    let mut dp = vec![vec![0; tlen + 1]; slen + 1];
    for i in 0..slen {
        for j in 0..tlen {
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i + 1][j]);
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j + 1]);
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
            }
        }
    }
    println!("{}", dp[slen][tlen]);
}

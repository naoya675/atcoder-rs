use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i + 1][j]);
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j + 1]);
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
            }
        }
    }
    let mut res = vec![];
    let mut i = s.len();
    let mut j = t.len();
    while i > 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1;
            continue;
        }
        if dp[i][j] == dp[i][j - 1] {
            j -= 1;
            continue;
        }
        res.push(s[i - 1]);
        i -= 1;
        j -= 1;
    }
    println!("{}", res.iter().rev().collect::<String>());
}

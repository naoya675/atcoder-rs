use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let slen = s.len();
    let tlen = t.len();
    let mut dp = vec![vec![1_000_000_000; tlen + 1]; slen + 1];
    let insert = 1;
    let delete = 1;
    let change = 1;
    for i in 0..=slen {
        dp[i][0] = i * insert;
    }
    for j in 0..=tlen {
        dp[0][j] = j * insert;
    }
    for i in 0..slen {
        for j in 0..tlen {
            dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j + 1] + delete);
            dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i + 1][j] + insert);
            dp[i + 1][j + 1] =
                dp[i + 1][j + 1].min(dp[i][j] + if s[i] == t[j] { 0 } else { change });
        }
    }
    println!("{}", dp[slen][tlen]);
}

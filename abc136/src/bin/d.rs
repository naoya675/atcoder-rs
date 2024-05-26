use itertools::Itertools;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut dp = vec![vec![0; 100]; s.len()];
    for i in 0..s.len() {
        match s[i] {
            'L' => dp[i][0] = i - 1,
            'R' => dp[i][0] = i + 1,
            _ => unreachable!(),
        }
    }
    for i in 0..30 {
        for j in 0..s.len() {
            dp[j][i + 1] = dp[dp[j][i]][i];
        }
    }
    let k = 1_000_000_000;
    let mut res = vec![0; s.len()];
    for i in 0..s.len() {
        let mut pos = i;
        for i in 0..30 {
            if k & (1 << i) != 0 {
                pos = dp[pos][i];
            }
        }
        res[pos] += 1;
    }
    println!("{}", res.iter().join(" "));
}

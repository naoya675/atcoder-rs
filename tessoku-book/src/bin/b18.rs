use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=s {
            dp[i + 1][j] |= dp[i][j];
            if j + a[i] <= s {
                dp[i + 1][j + a[i]] |= dp[i][j];
            }
        }
    }
    if dp[n][s] {
        let mut res = vec![];
        let mut i = n;
        let mut j = s;
        loop {
            if i == 0 {
                break;
            }
            if j >= a[i - 1] && dp[i - 1][j - a[i - 1]] {
                res.push(i);
                j -= a[i - 1];
                i -= 1;
            } else {
                i -= 1;
            }
        }
        res.reverse();
        println!("{}", res.len());
        println!("{}", res.iter().join(" "));
    } else {
        println!("{}", -1);
    }
}

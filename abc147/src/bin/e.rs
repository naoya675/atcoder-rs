use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }
    let max = (h + w) * 80;
    let mut dp = vec![vec![vec![false; max * 2]; w]; h];
    dp[0][0][a[0][0].abs_diff(b[0][0])] = true;
    for i in 0..h {
        for j in 0..w {
            let d = a[i][j].max(b[i][j]) - a[i][j].min(b[i][j]);
            for k in 0..max {
                if i > 0 {
                    dp[i][j][k] |= dp[i - 1][j][k + d];
                    dp[i][j][k] |= dp[i - 1][j][k.abs_diff(d)];
                }
                if j > 0 {
                    dp[i][j][k] |= dp[i][j - 1][k + d];
                    dp[i][j][k] |= dp[i][j - 1][k.abs_diff(d)];
                }
            }
        }
    }
    let mut res = max;
    for k in 0..max {
        if dp[h - 1][w - 1][k] {
            res = res.min(k);
        }
    }
    println!("{}", res);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let v = 100_000;
    let mut dp = vec![vec![1_000_000_001; v + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (wi, vi) = wv[i];
        for j in 0..=v {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            if j + vi <= v {
                dp[i + 1][j + vi] = dp[i + 1][j + vi].min(dp[i][j] + wi);
            }
        }
    }
    let mut res = 0;
    for j in 0..=v {
        if dp[n][j] <= w {
            res = res.max(j);
        }
    }
    println!("{}", res);
}

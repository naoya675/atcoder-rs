use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 0..n {
        let (wi, vi) = wv[i];
        for j in 0..=w {
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            if j + wi <= w {
                dp[i + 1][j + wi] = dp[i + 1][j + wi].max(dp[i][j] + vi);
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}

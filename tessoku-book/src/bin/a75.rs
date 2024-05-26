use proconio::input;

fn main() {
    input! {
        n: usize,
        mut td: [(usize, usize); n],
    }
    td.sort_by_key(|f| f.1);
    let mut dp = vec![vec![0; 2_000]; n + 1];
    for i in 0..n {
        let (t, d) = td[i];
        for j in 0..2_000 {
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            if j + t <= d {
                dp[i + 1][j + t] = dp[i + 1][j + t].max(dp[i][j] + 1);
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut ab: [(usize, i32); n],
    }
    ab.sort();
    let mut dp = vec![vec![0; t + 1]; n + 1];
    let mut res = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..t {
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            dp[i + 1][(j + a).min(t)] = dp[i + 1][(j + a).min(t)].max(dp[i][j] + b);
        }
        res = res.max(dp[i + 1][t]);
    }
    println!("{}", res);
}

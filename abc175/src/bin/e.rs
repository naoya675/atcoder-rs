use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        rcv: [(usize, usize, i64); k],
    }
    let mut vec = vec![vec![0; c]; r];
    for (r, c, v) in rcv {
        vec[r - 1][c - 1] = v;
    }
    let mut dp = vec![vec![vec![0; 4]; c]; r];
    for i in 0..r {
        for j in 0..c {
            for k in (0..3).rev() {
                dp[i][j][k + 1] = dp[i][j][k + 1].max(dp[i][j][k] + vec[i][j]);
            }
            for k in 0..4 {
                if i + 1 < r {
                    dp[i + 1][j][0] = dp[i + 1][j][0].max(dp[i][j][k]);
                }
                if j + 1 < c {
                    dp[i][j + 1][k] = dp[i][j + 1][k].max(dp[i][j][k]);
                }
            }
        }
    }
    println!("{}", dp[r - 1][c - 1].iter().max().unwrap());
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); m],
    }
    let mut score = vec![vec![0; n]; n];
    for (a, b) in ab {
        for i in 0..n {
            for j in 0..=i {
                if j <= a - 1 && b - 1 <= i {
                    score[j][i] += 1;
                }
            }
        }
    }
    let mut dp = vec![vec![-1_000_000_000; n + 1]; k + 1];
    dp[0][0] = 0;
    for k in 0..k {
        for i in 0..n {
            for j in 0..=i {
                dp[k + 1][i + 1] = dp[k + 1][i + 1].max(dp[k][j] + score[j][i]);
            }
        }
    }
    println!("{}", dp[k][n]);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }
    let mut dp = vec![vec![1_000_000_000; 1 << n]; m + 1];
    dp[0][0] = 0;
    for i in 0..m {
        let mut bit = 0;
        for j in 0..n {
            if a[i][j] == 1 {
                bit |= 1 << j;
            }
        }
        for j in 0..1 << n {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            dp[i + 1][j | bit] = dp[i + 1][j | bit].min(dp[i][j] + 1);
        }
    }
    if dp[m][(1 << n) - 1] <= m {
        println!("{}", dp[m][(1 << n) - 1]);
    } else {
        println!("{}", -1);
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; 2 * n],
    }
    let mut dp = vec![vec![1_000_000_000; 2 * n]; 2 * n];
    for i in 1..2 * n {
        dp[i - 1][i] = (a[i - 1] - a[i]).abs();
    }
    for k in 2..2 * n {
        for i in 0..2 * n - k {
            let j = i + k;
            dp[i][j] = dp[i][j].min(dp[i + 1][j - 1] + (a[i] - a[j]).abs());
            for m in i..j {
                dp[i][j] = dp[i][j].min(dp[i][m] + dp[m + 1][j]);
            }
        }
    }
    println!("{}", dp[0][2 * n - 1]);
}

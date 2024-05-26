use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    }
    let mut dp = vec![vec![0; n]; n];
    for k in (0..n - 1).rev() {
        for i in 0..n - k {
            let j = i + k;
            if i >= 1 {
                let (pi, ai) = pa[i - 1];
                let score = if i <= pi - 1 && pi - 1 <= j { ai } else { 0 };
                dp[i][j] = dp[i][j].max(dp[i - 1][j] + score);
            }
            if j + 1 < n {
                let (pi, ai) = pa[j + 1];
                let score = if i <= pi - 1 && pi - 1 <= j { ai } else { 0 };
                dp[i][j] = dp[i][j].max(dp[i][j + 1] + score);
            }
        }
    }
    let mut res = 0;
    for i in 0..n {
        res = res.max(dp[i][i]);
    }
    println!("{}", res);
}

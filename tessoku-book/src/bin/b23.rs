use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }
    let mut dp = vec![vec![1_000_000_000_f64; n + 1]; 1 << n];
    dp[0][0] = 0.;
    for bit in 0..1 << n {
        for i in 0..n {
            for j in 0..n {
                let (xi, yi) = xy[i];
                let (xj, yj) = xy[j];
                if bit & 1 << j == 0 {
                    let d = ((xi - xj) * (xi - xj) + (yi - yj) * (yi - yj)).sqrt();
                    dp[bit | 1 << j][j] = dp[bit | 1 << j][j].min(dp[bit][i] + d);
                }
            }
        }
    }
    println!("{}", dp[(1 << n) - 1][0]);
}

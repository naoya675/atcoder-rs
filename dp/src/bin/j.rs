use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut dp = vec![vec![vec![0.; n + 1]; n + 1]; n + 1];
    for k in 0..=n {
        for j in 0..=n {
            for i in 0..=n {
                if i + j + k == 0 {
                    continue;
                }
                if i + j + k > n {
                    continue;
                }
                let mut res = n as f64;
                if i > 0 {
                    res += dp[i - 1][j][k] * i as f64;
                }
                if j > 0 {
                    res += dp[i + 1][j - 1][k] * j as f64;
                }
                if k > 0 {
                    res += dp[i][j + 1][k - 1] * k as f64;
                }
                dp[i][j][k] = res / (i + j + k) as f64;
            }
        }
    }
    let cnt1 = a.iter().filter(|&&a| a == 1).count();
    let cnt2 = a.iter().filter(|&&a| a == 2).count();
    let cnt3 = a.iter().filter(|&&a| a == 3).count();
    println!("{}", dp[cnt1][cnt2][cnt3]);
}

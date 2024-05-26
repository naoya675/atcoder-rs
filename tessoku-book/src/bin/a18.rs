use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=s {
            dp[i + 1][j] |= dp[i][j];
            if j + a[i] <= s {
                dp[i + 1][j + a[i]] |= dp[i][j];
            }
        }
    }
    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}

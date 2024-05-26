use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i32; n],
    }
    let mut dp = vec![1 << 30; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 0..k {
            if i + j + 1 < n {
                dp[i + j + 1] = dp[i + j + 1].min(dp[i] + (h[i + j + 1] - h[i]).abs())
            }
        }
    }
    println!("{}", dp[n - 1]);
}

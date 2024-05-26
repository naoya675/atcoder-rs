use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n - 1],
        b: [i32; n - 2],
    }
    let mut dp = vec![1_000_000_000; n];
    dp[0] = 0;
    for i in 0..n {
        if i + 1 < n {
            dp[i + 1] = dp[i + 1].min(dp[i] + a[i]);
        }
        if i + 2 < n {
            dp[i + 2] = dp[i + 2].min(dp[i] + b[i]);
        }
    }
    println!("{}", dp[n - 1]);
}

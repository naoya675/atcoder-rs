use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let modulus: i64 = 1_000_000_007;
    let mut dp = vec![0; n + 1];
    let mut ng = vec![false; n + 1];
    for a in a {
        ng[a] = true;
    }
    dp[0] = 1;
    for i in 0..n {
        if ng[i] {
            continue;
        }
        if i + 1 <= n {
            dp[i + 1] = (dp[i + 1] + dp[i]) % modulus;
            // dp[i + 1] += dp[i];
            // dp[i + 1] %= modulus;
        }
        if i + 2 <= n {
            dp[i + 2] = (dp[i + 2] + dp[i]) % modulus;
            // dp[i + 2] += dp[i];
            // dp[i + 2] %= modulus;
        }
    }
    println!("{}", dp[n]);
}

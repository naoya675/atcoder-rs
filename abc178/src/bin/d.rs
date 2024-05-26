use proconio::input;

fn main() {
    input! {
        s: usize,
    }
    let modulus: i64 = 1_000_000_007;
    let mut dp = vec![0; 2002];
    dp[0] = 1;
    dp[1] = 0;
    dp[2] = 0;
    for i in 3..=s {
        dp[i] = (dp[i - 1] + dp[i - 3]) % modulus;
    }
    println!("{}", dp[s]);
}

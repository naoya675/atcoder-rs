use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let modulus: i64 = 1_000_000_007;
    let mut dp = vec![vec![0; 13]; s.len() + 1];
    dp[0][0] = 1;
    for i in 0..s.len() {
        for j in 0..13 {
            for k in 0..10 {
                if s[i] == '?' || s[i] as usize - '0' as usize == k {
                    dp[i + 1][(j * 10 + k) % 13] =
                        (dp[i + 1][(j * 10 + k) % 13] + dp[i][j]) % modulus;
                }
            }
        }
    }
    println!("{}", dp[s.len()][5]);
}

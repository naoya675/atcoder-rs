use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }
    let modulus: i64 = 1_000_000_007;
    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                continue;
            }
            if i + 1 < h {
                dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % modulus;
            }
            if j + 1 < w {
                dp[i][j + 1] = (dp[i][j + 1] + dp[i][j]) % modulus;
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}

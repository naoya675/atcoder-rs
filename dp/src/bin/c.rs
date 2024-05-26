use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [[i32; 3]; n],
    }
    let mut dp = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                if j == k {
                    continue;
                }
                dp[i + 1][k] = dp[i + 1][k].max(dp[i][j] + abc[i][j]);
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}

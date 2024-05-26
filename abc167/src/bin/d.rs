use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![0; 100]; n];
    for i in 0..n {
        dp[i][0] = a[i] - 1;
    }
    for i in 0..60 {
        for j in 0..n {
            dp[j][i + 1] = dp[dp[j][i]][i];
        }
    }
    let mut res = 0;
    for i in 0..60 {
        if k & (1 << i) != 0 {
            res = dp[res][i];
        }
    }
    println!("{}", res + 1);
}

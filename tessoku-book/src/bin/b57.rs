use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut dp = vec![vec![0; 100]; n + 1];
    for i in 1..=n {
        let mut sum = 0;
        let mut j = i;
        while j > 0 {
            sum += j % 10;
            j /= 10;
        }
        dp[i][0] = i - sum;
    }
    for i in 0..30 {
        for j in 1..=n {
            dp[j][i + 1] = dp[dp[j][i]][i];
        }
    }
    for i in 1..=n {
        let mut res = i;
        for i in 0..30 {
            if k & (1 << i) != 0 {
                res = dp[res][i];
            }
        }
        println!("{}", res);
    }
}

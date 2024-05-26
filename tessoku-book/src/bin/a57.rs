use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xy: [(usize, usize); q],
    }
    let mut dp = vec![vec![0; 100]; n];
    for i in 0..n {
        dp[i][0] = a[i] - 1;
    }
    for i in 0..30 {
        for j in 0..n {
            dp[j][i + 1] = dp[dp[j][i]][i];
        }
    }
    for (x, y) in xy {
        let mut res = x - 1;
        for i in 0..30 {
            if y & (1 << i) != 0 {
                res = dp[res][i];
            }
        }
        println!("{}", res + 1);
    }
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        (r, s, p): (i32, i32, i32),
        t: Chars,
    }
    let mut res = 0;
    let mut select = vec![false; n];
    for i in 0..n {
        if i >= k && t[i] == t[i - k] && select[i - k] == false {
            select[i] = true;
        } else {
            res += match t[i] {
                'r' => p,
                's' => r,
                'p' => s,
                _ => unreachable!(),
            }
        }
    }
    // let mut sep = vec![vec![]; k];
    // for i in 0..n {
    //     sep[i % k].push(t[i]);
    // }
    // for sep in sep {
    //     let mut dp = vec![vec![0; 3]; sep.len() + 1];
    //     for i in 0..sep.len() {
    //         for j in 0..3 {
    //             for k in 0..3 {
    //                 dp[i + 1][k] = dp[i + 1][k].max(dp[i][j]);
    //             }
    //         }
    //         match sep[i] {
    //             'r' => {
    //                 dp[i + 1][1] = dp[i + 1][1].max(dp[i][0] + p);
    //                 dp[i + 1][2] = dp[i + 1][2].max(dp[i][0] + p);
    //             }
    //             's' => {
    //                 dp[i + 1][0] = dp[i + 1][0].max(dp[i][1] + r);
    //                 dp[i + 1][2] = dp[i + 1][2].max(dp[i][1] + r);
    //             }
    //             'p' => {
    //                 dp[i + 1][0] = dp[i + 1][0].max(dp[i][2] + s);
    //                 dp[i + 1][1] = dp[i + 1][1].max(dp[i][2] + s);
    //             }
    //             _ => unreachable!(),
    //         }
    //     }
    //     res += dp[sep.len()].iter().max().unwrap();
    // }
    println!("{}", res);
}

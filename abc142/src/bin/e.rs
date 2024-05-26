use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut ac = vec![];
    for _ in 0..m {
        input! {
            a: i32,
            b: usize,
            c: [usize; b],
        }
        let bit = c.iter().fold(0, |bit, c| bit + (1 << (c - 1)));
        ac.push((a, bit));
    }
    let mut dp = vec![vec![1 << 30; m + 1]; 1 << n];
    dp[0][0] = 0;
    for (i, (a, bit)) in ac.iter().enumerate() {
        for j in 0..1 << n {
            dp[j][i + 1] = dp[j][i + 1].min(dp[j][i]);
            dp[j | bit][i + 1] = dp[j | bit][i + 1].min(dp[j][i] + a);
        }
    }
    let res = dp[(1 << n) - 1][m];
    println!("{}", if res == 1 << 30 { -1 } else { res });
}

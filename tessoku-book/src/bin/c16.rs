use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut asbt: [(usize, usize, usize, usize); m],
    }
    let asbt = asbt
        .into_iter()
        .map(|(a, s, b, t)| (a, s, b, t + k))
        .sorted_by_key(|f| f.3);
    let mut dp = vec![vec![(0, 0)]; n];
    for (a, s, b, t) in asbt {
        let p = dp[a - 1].partition_point(|pred| pred.0 <= s);
        let count = (dp[a - 1][p - 1].1 + 1).max(dp[b - 1].last().unwrap().1);
        dp[b - 1].push((t, count));
    }
    let res = dp.iter().map(|f| f.last().unwrap().1).max().unwrap();
    println!("{}", res);
}

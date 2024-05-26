use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i32, i32); n],
    }
    xy.sort_by_key(|f| (f.0, -f.1));
    let mut dp = vec![0; n];
    let mut l = vec![1_000_000_000; n + 1];
    l[0] = 0;
    for i in 0..n {
        let (_, y) = xy[i];
        let j = l.lower_bound(&y);
        dp[i] = j;
        l[j] = l[j].min(y);
    }
    let mut res = 0;
    for i in 0..n {
        res = res.max(dp[i]);
    }
    println!("{}", res);
}

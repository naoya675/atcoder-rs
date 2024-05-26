use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut dp = vec![0; n];
    let mut l = vec![1_000_000_000; n + 1];
    l[0] = 0;
    for i in 0..n {
        let j = l.lower_bound(&a[i]);
        dp[i] = j;
        l[j] = l[j].min(a[i]);
    }
    let mut res = 0;
    for i in 0..n {
        res = res.max(dp[i]);
    }
    println!("{}", res);
}

use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: isize,
        l: isize,
        r: isize,
        mut x: [isize; n],
    }
    let m = 1_000_000_007;
    x.push(0);
    x.push(w);
    x.sort();
    let mut dp = vec![0; n + 2];
    let mut sum = vec![0; n + 2];
    dp[0] = 1;
    sum[1] = 1;
    for i in 1..n + 2 {
        // let pl = x.partition_point(|&f| f < x[i] - r);
        // let pr = x.partition_point(|&f| f <= x[i] - l);
        let pl = x.lower_bound(&(x[i] - r));
        let pr = x.lower_bound(&(x[i] - l + 1));
        dp[i] = (sum[pr] - sum[pl] + m) % m;
        if i < n + 1 {
            sum[i + 1] = (sum[i] + dp[i]) % m;
        }
    }
    println!("{}", dp[n + 1]);
}

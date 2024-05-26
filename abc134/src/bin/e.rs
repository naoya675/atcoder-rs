use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut set = BTreeSet::new();
    for (i, &a) in a.iter().enumerate() {
        if let Some(x) = set.range(..(a, 0)).next_back().cloned() {
            set.remove(&x);
        }
        set.insert((a, i));
    }
    println!("{}", set.len());

    // a.reverse();
    // let mut dp = vec![0; n];
    // let mut l = vec![1_000_000_001; n + 1];
    // l[0] = -1;
    // for i in 0..n {
    //     let j = l.lower_bound(&(a[i] + 1));
    //     dp[i] = j;
    //     l[j] = l[j].min(a[i]);
    // }
    // let mut res = 0;
    // for i in 0..n {
    //     res = res.max(dp[i]);
    // }
    // println!("{}", res);
}

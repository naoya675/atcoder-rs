use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [f64; n],
    }
    let mut ok = 1_000_000_000.;
    let mut ng = 0.;
    while ok - ng > 1e-6 {
        let mi = (ok + ng) / 2.;
        let seat = a.iter().map(|f| (f / mi) as i64).sum::<i64>();
        // let mut seat = 0;
        // for i in 0..n {
        //     seat += (a[i] / mi) as i64;
        // }
        if seat > k {
            ng = mi;
        } else {
            ok = mi;
        }
    }
    let res = a.iter().map(|f| (f / ok) as i64).collect::<Vec<_>>();
    // let mut res = vec![0; n];
    // for i in 0..n {
    //     res[i] = (a[i] / ok) as i64;
    // }
    println!("{}", res.iter().join(" "));
}

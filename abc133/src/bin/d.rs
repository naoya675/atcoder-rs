use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut res = vec![0; n];
    for i in 0..n {
        if i % 2 == 0 {
            res[0] += a[i];
        } else {
            res[0] -= a[i];
        }
    }
    for i in 1..n {
        res[i] = a[i - 1] * 2 - res[i - 1];
    }
    println!("{}", res.iter().join(" "));
}

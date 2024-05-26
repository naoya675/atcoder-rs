use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut cnt = vec![0; n];
    for i in (0..n).rev() {
        let mut count = 0;
        for j in (i..n).step_by(i + 1) {
            count ^= cnt[j];
        }
        if count != a[i] {
            cnt[i] = 1;
        }
    }
    let mut res = vec![];
    for i in 0..n {
        if cnt[i] == 1 {
            res.push(i + 1);
        }
    }
    println!("{}", res.len());
    println!("{}", res.iter().join(" "));
}

use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut res = false;
    for i in a.iter().cloned().combinations(3) {
        if i.iter().sum::<i32>() == 1000 {
            res = true;
        }
    }
    // for i in 0..n {
    //     for j in i + 1..n {
    //         for k in j + 1..n {
    //             if a[i] + a[j] + a[k] == 1000 {
    //                 res = true;
    //             }
    //         }
    //     }
    // }
    println!("{}", if res { "Yes" } else { "No" });
}

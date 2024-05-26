use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    // let mut res = vec![0; n];
    // for i in 0..n {
    //     res[a[i] - 1] = i + 1;
    // }
    let res = a
        .iter()
        .enumerate()
        .sorted_by_key(|&(_, a)| a)
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();
    println!("{}", res.iter().join(" "));
}

use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
    }
    let mut res = vec![0; n];
    for a in a {
        res[a - 1] += 1;
    }
    println!("{}", res.iter().join("\n"));
}

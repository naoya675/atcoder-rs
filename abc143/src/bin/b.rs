use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i32; n],
    }
    let res = d.iter().combinations(2).map(|d| d[0] * d[1]).sum::<i32>();
    println!("{}", res);
}

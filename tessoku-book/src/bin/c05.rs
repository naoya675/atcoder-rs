use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    n -= 1;
    let mut res = vec![];
    for i in 0..10 {
        if n & (1 << i) == 0 {
            res.push(4);
        } else {
            res.push(7);
        }
    }
    println!("{}", res.iter().rev().join(""));
}

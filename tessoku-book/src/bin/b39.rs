use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut xy: [(usize, usize); n],
    }
    xy.sort();
    let mut queue = BinaryHeap::new();
    let mut res = 0;
    let mut j = 0;
    for i in 1..=d {
        while j < n && xy[j].0 <= i {
            queue.push(xy[j].1);
            j += 1;
        }
        if let Some(v) = queue.pop() {
            res += v;
        }
    }
    println!("{}", res);
}

use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    let mut heap = BinaryHeap::new();
    for a in a {
        heap.push(a);
    }
    for _ in 0..m {
        if let Some(v) = heap.pop() {
            heap.push(v / 2);
        }
    }
    let res = heap.iter().sum::<i64>();
    println!("{}", res);
}

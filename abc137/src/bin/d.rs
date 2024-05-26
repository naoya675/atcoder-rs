use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, i32); n],
    }
    let mut vec = vec![vec![]; m];
    for (a, b) in ab {
        if a <= m {
            vec[m - a].push(b);
        }
    }
    let mut queue = BinaryHeap::new();
    let mut res = 0;
    for i in (0..m).rev() {
        for b in vec[i].iter() {
            queue.push(b);
        }
        if let Some(v) = queue.pop() {
            res += v;
        }
    }
    println!("{}", res);
}

use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(usize, usize, i32, i32); m],
    }
    let p = 10_000;
    let mut graph = vec![vec![]; n];
    for (a, b, c, d) in abcd {
        graph[a - 1].push((b - 1, c * p - d));
        graph[b - 1].push((a - 1, c * p - d));
    }
    let mut res = vec![1_000_000_000; n];
    let mut queue = BinaryHeap::new();
    res[0] = 0;
    queue.push((res[0], 0));
    while let Some((d, v)) = queue.pop() {
        if res[v] < -d {
            continue;
        }
        for &(i, c) in &graph[v] {
            if res[i] > res[v] + c {
                res[i] = res[v] + c;
                queue.push((-res[i], i));
            }
        }
    }
    let d = (res[n - 1] + p - 1) / p;
    let t = d * p - res[n - 1];
    println!("{} {}", d, t);
}

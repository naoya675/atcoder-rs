use itertools::Itertools;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, i32); m],
    }
    let mut graph = vec![vec![]; n];
    for (a, b, c) in abc {
        graph[a - 1].push((b - 1, c));
        graph[b - 1].push((a - 1, c));
    }
    let mut dist = vec![1_000_000_000; n];
    let mut prev = vec![0; n];
    let mut queue = BinaryHeap::new();
    dist[0] = 0;
    queue.push((-dist[0], 0));
    while let Some((d, v)) = queue.pop() {
        if dist[v] < -d {
            continue;
        }
        for &(i, c) in &graph[v] {
            if dist[i] > dist[v] + c {
                dist[i] = dist[v] + c;
                prev[i] = v;
                queue.push((-dist[i], i));
            }
        }
    }
    let mut v = n - 1;
    let mut res = vec![v + 1];
    while v != 0 {
        v = prev[v];
        res.push(v + 1);
    }
    println!("{}", res.iter().rev().join(" "));
}

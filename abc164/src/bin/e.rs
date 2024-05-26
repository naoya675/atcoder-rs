use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        uvab: [(usize, usize, usize, i64); m],
        cd: [(usize, i64); n],
    }
    let max = 50 * 50;
    let s = s.min(max);
    let mut graph = vec![vec![]; n];
    for (u, v, a, b) in uvab {
        graph[u - 1].push((v - 1, a, b));
        graph[v - 1].push((u - 1, a, b));
    }
    let mut dp = vec![vec![1 << 60; max + 1]; n];
    dp[0][s] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(dp[0][s]), 0, s));
    while let Some((Reverse(d), from, c)) = heap.pop() {
        if dp[from][c] < d {
            continue;
        }
        let (cf, df) = cd[from];
        let nc = (c + cf).min(max);
        let nd = d + df;
        if dp[from][nc] > nd {
            dp[from][nc] = nd;
            heap.push((Reverse(dp[from][nc]), from, nc));
        }
        for &(to, a, b) in &graph[from] {
            if c < a {
                continue;
            }
            let nc = c - a;
            let nd = d + b;
            if dp[to][nc] > nd {
                dp[to][nc] = nd;
                heap.push((Reverse(dp[to][nc]), to, nc));
            }
        }
    }
    for i in 1..n {
        println!("{}", dp[i].iter().min().unwrap());
    }
}

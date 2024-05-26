use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for (x, y) in xy {
        graph[x - 1].push(y - 1);
    }
    let mut dp = vec![0; n];
    for i in topological_sort(n, &graph) {
        for &j in &graph[i] {
            dp[j] = dp[j].max(dp[i] + 1);
        }
    }
    println!("{}", dp.iter().max().unwrap());
}

pub fn topological_sort(size: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut indegree = vec![0; size];
    for from in 0..size {
        for i in 0..graph[from].len() {
            let to = graph[from][i];
            indegree[to] += 1;
        }
    }
    let mut heap = BinaryHeap::new();
    for from in 0..size {
        if indegree[from] == 0 {
            heap.push(Reverse(from));
        }
    }
    let mut res = vec![];
    while let Some(Reverse(from)) = heap.pop() {
        res.push(from);
        for i in 0..graph[from].len() {
            let to = graph[from][i];
            indegree[to] -= 1;
            if indegree[to] == 0 {
                heap.push(Reverse(to));
            }
        }
    }
    if res.len() != size {
        return vec![];
    }
    res
}

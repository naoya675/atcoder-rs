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
    let mut dist1 = vec![1_000_000_000; n];
    let mut dist2 = vec![1_000_000_000; n];
    let mut queue = BinaryHeap::new();
    dist1[0] = 0;
    queue.push((-dist1[0], 0));
    while let Some((d, v)) = queue.pop() {
        if dist1[v] < -d {
            continue;
        }
        for &(i, c) in &graph[v] {
            if dist1[i] > dist1[v] + c {
                dist1[i] = dist1[v] + c;
                queue.push((-dist1[i], i));
            }
        }
    }
    dist2[n - 1] = 0;
    queue.push((-dist2[n - 1], n - 1));
    while let Some((d, v)) = queue.pop() {
        if dist2[v] < -d {
            continue;
        }
        for &(i, c) in &graph[v] {
            if dist2[i] > dist2[v] + c {
                dist2[i] = dist2[v] + c;
                queue.push((-dist2[i], i));
            }
        }
    }
    let mut res = 0;
    for i in 0..n {
        if dist1[n - 1] == dist1[i] + dist2[i] {
            res += 1;
        }
    }
    println!("{}", res);
}

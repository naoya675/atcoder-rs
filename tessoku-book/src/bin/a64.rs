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
    let mut res = vec![1_000_000_000; n];
    let mut queue = BinaryHeap::new();
    res[0] = 0;
    queue.push((-res[0], 0));
    while !queue.is_empty() {
        let (d, v) = queue.pop().unwrap();
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
    for i in 0..n {
        println!("{}", if res[i] == 1_000_000_000 { -1 } else { res[i] });
    }
}

use std::collections::VecDeque;

use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut check = vec![0; n];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(v) = queue.pop_front() {
        for &next_v in &graph[v] {
            if check[next_v] == 0 {
                check[next_v] = v + 1;
                queue.push_back(next_v);
            }
        }
    }
    if check[1..].iter().all(|&c| c != 0) {
        println!("Yes");
        println!("{}", check[1..].iter().join("\n"));
    } else {
        println!("No");
    }
}

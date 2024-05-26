use std::collections::VecDeque;

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
    let mut res = vec![-1; n];
    let mut queue = VecDeque::new();
    res[0] = 0;
    queue.push_back(0);
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        for &i in &graph[v] {
            if res[i] == -1 {
                res[i] = res[v] + 1;
                queue.push_back(i);
            }
        }
    }
    for i in 0..n {
        println!("{}", res[i]);
    }
}

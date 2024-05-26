use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
        s: usize,
        t: usize,
    }
    let mut graph = vec![vec![]; n * 3];
    for (u, v) in uv {
        graph[(u - 1) * 3 + 0].push((v - 1) * 3 + 1);
        graph[(u - 1) * 3 + 1].push((v - 1) * 3 + 2);
        graph[(u - 1) * 3 + 2].push((v - 1) * 3 + 0);
    }
    let mut res = vec![-1; n * 3];
    let mut queue = VecDeque::new();
    res[(s - 1) * 3] = 0;
    queue.push_back((s - 1) * 3);
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        for &i in &graph[v] {
            if res[i] == -1 {
                res[i] = res[v] + 1;
                queue.push_back(i);
            }
        }
    }
    if res[(t - 1) * 3] % 3 == 0 {
        println!("{}", res[(t - 1) * 3] / 3);
    } else {
        println!("-1");
    }
}

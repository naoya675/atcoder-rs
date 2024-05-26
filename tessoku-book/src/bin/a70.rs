use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        xyz: [(usize, usize, usize); m],
    }
    let mut graph = vec![vec![]; 1 << n];
    for i in 0..1 << n {
        for (x, y, z) in &xyz {
            let j = i ^ (1 << x - 1) ^ (1 << y - 1) ^ (1 << z - 1);
            graph[i].push(j);
        }
    }
    let mut start = 0;
    for i in 0..n {
        if a[i] == 1 {
            start |= 1 << i;
        }
    }
    let mut res = vec![-1; 1 << n];
    let mut queue = VecDeque::new();
    res[start] = 0;
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for &i in &graph[v] {
            if res[i] == -1 {
                res[i] = res[v] + 1;
                queue.push_back(i);
            }
        }
    }
    println!("{}", res[(1 << n) - 1]);
}

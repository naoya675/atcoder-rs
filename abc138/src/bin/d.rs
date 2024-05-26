use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        px: [(usize, i32); q],
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut res = vec![0; n];
    for (p, x) in px {
        res[p - 1] += x;
    }
    dfs(&graph, &mut res, 0, 0);
    println!("{}", res.iter().join(" "));
}

fn dfs(graph: &Vec<Vec<usize>>, res: &mut Vec<i32>, v: usize, prev_v: usize) {
    for &i in &graph[v] {
        if i != prev_v {
            res[i] += res[v];
            dfs(graph, res, i, v);
        }
    }
}

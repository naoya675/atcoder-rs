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
    let mut check = vec![false; n];
    let mut res = vec![];
    dfs(&graph, &mut check, &mut res, 0, n - 1);
    println!("{}", res.iter().rev().join(" "));
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    check: &mut Vec<bool>,
    res: &mut Vec<usize>,
    v: usize,
    t: usize,
) -> bool {
    check[v] = true;
    if v == t {
        res.push(v + 1);
        return true;
    }
    for &i in &graph[v] {
        if check[i] == false {
            if dfs(graph, check, res, i, t) {
                res.push(v + 1);
                return true;
            }
        }
    }
    false
}

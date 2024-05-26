use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut check = vec![false; n];
    let mut res = vec![0; n];
    dfs(&graph, &mut check, &mut res, t - 1);
    println!("{}", res.iter().join(" "));
}

fn dfs(graph: &Vec<Vec<usize>>, check: &mut Vec<bool>, res: &mut Vec<usize>, v: usize) -> usize {
    check[v] = true;
    for &i in &graph[v] {
        if check[i] == false {
            res[v] = res[v].max(dfs(graph, check, res, i) + 1);
        }
    }
    res[v]
}

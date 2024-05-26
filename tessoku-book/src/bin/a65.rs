use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
    }
    let mut graph = vec![vec![]; n];
    for i in 1..n {
        graph[a[i - 1] - 1].push(i);
    }
    let mut res = vec![0; n];
    dfs(&graph, &mut res, 0);
    println!("{}", res.iter().join(" "));
}

fn dfs(graph: &Vec<Vec<usize>>, res: &mut Vec<usize>, v: usize) -> usize {
    for &i in &graph[v] {
        res[v] += dfs(graph, res, i) + 1;
    }
    res[v]
}

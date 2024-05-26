use proconio::input;

fn main() {
    input! {
        n: usize,
        uvm: [(usize, usize, usize); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for (u, v, m) in uvm {
        graph[u - 1].push((v - 1, m));
        graph[v - 1].push((u - 1, m));
    }
    let mut check = vec![-1; n];
    check[0] = 0;
    dfs(&graph, &mut check, 0);
    for i in 0..n {
        println!("{}", check[i]);
    }
}

fn dfs(graph: &Vec<Vec<(usize, usize)>>, check: &mut Vec<i32>, v: usize) {
    for &(i, m) in &graph[v] {
        if check[i] == -1 {
            check[i] = if m % 2 == 0 { check[v] } else { 1 - check[v] };
            dfs(graph, check, i)
        }
    }
}

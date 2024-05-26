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
    dfs(&graph, &mut check, 0);
    if check.iter().all(|&f| f) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}

fn dfs(graph: &Vec<Vec<usize>>, check: &mut Vec<bool>, v: usize) {
    check[v] = true;
    for &i in &graph[v] {
        if check[i] == false {
            dfs(graph, check, i);
        }
    }
}

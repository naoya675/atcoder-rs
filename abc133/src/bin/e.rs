use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        ab: [(usize, usize); n - 1],
    }
    let modulus: i64 = 1_000_000_007;
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let res = (k * dfs(&graph, 0, 0, k, modulus)) % modulus;
    println!("{}", res);
}

fn dfs(graph: &Vec<Vec<usize>>, v: usize, prev_v: usize, k: i64, modulus: i64) -> i64 {
    let mut res = 1;
    let mut pattern = if v == 0 { k - 1 } else { k - 2 };
    for &i in &graph[v] {
        if prev_v != i {
            res = (res * pattern) % modulus;
            if pattern > 0 {
                pattern -= 1;
            }
            res = (res * dfs(graph, i, v, k, modulus)) % modulus;
        }
    }
    res
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for (i, (a, b)) in ab.iter().enumerate() {
        graph[a - 1].push((b - 1, i));
        graph[b - 1].push((a - 1, i));
    }
    let mut res = vec![0; n - 1];
    dfs(&graph, 0, 0, 0, &mut res);
    println!("{}", res.iter().max().unwrap());
    for i in 0..n - 1 {
        println!("{}", res[i]);
    }
}

fn dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    v: usize,
    prev_v: usize,
    prev_c: usize,
    res: &mut Vec<usize>,
) {
    let mut c = 1;
    for &(next_v, i) in &graph[v] {
        if prev_c == c {
            c += 1;
        }
        if prev_v != next_v {
            res[i] = c;
            dfs(graph, next_v, v, c, res);
            c += 1;
        }
    }
}

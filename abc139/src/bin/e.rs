use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n - 1]; n],
    }
    let mut graph = vec![vec![vec![]; n]; n];
    for i in 0..n {
        for j in 1..n - 1 {
            let p = i.min(a[i][j - 1] - 1);
            let q = i.max(a[i][j - 1] - 1);
            let r = i.min(a[i][j] - 1);
            let s = i.max(a[i][j] - 1);
            graph[p][q].push((r, s));
        }
    }
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        let p = i.min(a[i][0] - 1);
        let q = i.max(a[i][0] - 1);
        graph[0][0].push((p, q));
    }
    let mut pre = vec![vec![false; n]; n];
    let mut post = vec![vec![false; n]; n];
    let cycle = dfs(&graph, &mut res, &mut pre, &mut post, 0, 0);
    println!("{}", if cycle { -1 } else { res[0][0] });
}

fn dfs(
    graph: &Vec<Vec<Vec<(usize, usize)>>>,
    res: &mut Vec<Vec<i32>>,
    pre: &mut Vec<Vec<bool>>,
    post: &mut Vec<Vec<bool>>,
    p: usize,
    q: usize,
) -> bool {
    pre[p][q] = true;
    for &(s, t) in &graph[p][q] {
        if !post[s][t] {
            if pre[s][t] {
                return true;
            }
            if dfs(graph, res, pre, post, s, t) {
                return true;
            }
        }
        res[p][q] = res[p][q].max(res[s][t] + 1);
    }
    post[p][q] = true;
    false
}

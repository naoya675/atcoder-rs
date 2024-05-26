use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: i64,
        abc: [(usize, usize, i64); m],
        q: usize,
        st: [(usize, usize); q],
    }
    let mut edge = vec![];
    for (a, b, c) in abc {
        edge.push(Edge::new(a - 1, b - 1, c));
        edge.push(Edge::new(b - 1, a - 1, c));
    }
    let (_, res) = warshall_floyd(n, &edge);
    let mut edge = vec![];
    for i in 0..n {
        for j in 0..n {
            if res[i][j] <= l {
                edge.push(Edge::new(i, j, 1));
                edge.push(Edge::new(j, i, 1));
            }
        }
    }
    let (_, res) = warshall_floyd(n, &edge);
    for (s, t) in st {
        let res = res[s - 1][t - 1];
        println!("{}", if res < i64::MAX / 4 { res - 1 } else { -1 });
    }
}

pub type Cost = i64;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    from: usize,
    to: usize,
    cost: Cost,
}

impl Edge {
    pub fn new(from: usize, to: usize, cost: Cost) -> Self {
        Self { from, to, cost }
    }
}

pub fn warshall_floyd(size: usize, edge: &Vec<Edge>) -> (bool, Vec<Vec<Cost>>) {
    let mut dist = vec![vec![Cost::MAX / 2; size]; size];
    for i in 0..size {
        dist[i][i] = 0;
    }
    for edge in edge {
        dist[edge.from][edge.to] = edge.cost;
    }
    for k in 0..size {
        for i in 0..size {
            for j in 0..size {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j])
            }
        }
    }
    let mut cycle = false;
    for i in 0..size {
        if dist[i][i] < 0 {
            cycle = true;
        }
    }
    (cycle, dist)
}

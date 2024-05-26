use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: [Chars; n],
    }
    let mut ff = FordFulkerson::new(2 * n + 2);
    let s = 2 * n;
    let t = 2 * n + 1;
    for i in 0..n {
        for j in 0..n {
            if c[i][j] == '#' {
                ff.add_edge(i, j + n, 1);
            }
        }
    }
    for i in 0..n {
        ff.add_edge(s, i, 1);
        ff.add_edge(i + n, t, 1);
    }
    println!("{}", ff.flow(s, t));
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

impl Edge {
    fn new(to: usize, cap: usize, rev: usize) -> Self {
        Self { to, cap, rev }
    }
}

#[derive(Debug, Clone)]
struct FordFulkerson {
    size: usize,
    used: Vec<bool>,
    graph: Vec<Vec<Edge>>,
}

impl FordFulkerson {
    fn new(size: usize) -> Self {
        Self {
            size,
            used: vec![false; size],
            graph: vec![vec![]; size],
        }
    }

    fn add_edge(&mut self, a: usize, b: usize, c: usize) {
        let alen = self.graph[a].len();
        let blen = self.graph[b].len();
        self.graph[a].push(Edge::new(b, c, blen));
        self.graph[b].push(Edge::new(a, 0, alen));
    }

    fn dfs(&mut self, v: usize, t: usize, f: usize) -> usize {
        if v == t {
            return f;
        }
        self.used[v] = true;
        for i in 0..self.graph[v].len() {
            let u = self.graph[v][i];
            if u.cap == 0 {
                continue;
            }
            if self.used[u.to] {
                continue;
            }
            let flow = self.dfs(u.to, t, f.min(u.cap));
            if flow > 0 {
                self.graph[v][i].cap -= flow;
                self.graph[u.to][u.rev].cap += flow;
                return flow;
            }
        }
        0
    }

    fn flow(&mut self, s: usize, t: usize) -> usize {
        let mut total_flow = 0;
        loop {
            for i in 0..self.size {
                self.used[i] = false;
            }
            let flow = self.dfs(s, t, usize::MAX);
            if flow == 0 {
                break;
            }
            total_flow += flow;
        }
        total_flow
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [i32; n],
        ab: [(usize, usize); m],
    }
    let mut ff = FordFulkerson::new(n + 2);
    let mut offset = 0;
    for i in 0..n {
        if p[i] > 0 {
            offset += p[i];
            ff.add_edge(n, i, p[i]);
        } else {
            ff.add_edge(i, n + 1, -p[i]);
        }
    }
    for (a, b) in ab {
        ff.add_edge(a - 1, b - 1, 1_000_000_000);
    }
    println!("{}", offset - ff.flow(n, n + 1));
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    to: usize,
    cap: i32,
    rev: usize,
}

impl Edge {
    fn new(to: usize, cap: i32, rev: usize) -> Self {
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

    fn add_edge(&mut self, a: usize, b: usize, c: i32) {
        let alen = self.graph[a].len();
        let blen = self.graph[b].len();
        self.graph[a].push(Edge::new(b, c, blen));
        self.graph[b].push(Edge::new(a, 0, alen));
    }

    fn dfs(&mut self, v: usize, t: usize, f: i32) -> i32 {
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

    fn flow(&mut self, s: usize, t: usize) -> i32 {
        let mut total_flow = 0;
        loop {
            for i in 0..self.size {
                self.used[i] = false;
            }
            let flow = self.dfs(s, t, i32::MAX);
            if flow == 0 {
                break;
            }
            total_flow += flow;
        }
        total_flow
    }
}

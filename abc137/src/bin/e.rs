use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: i64,
        abc: [(usize, usize, i64); m],
    }
    let mut bf = BellmanFord::new(n);
    for (a, b, c) in abc {
        bf.add_edge(a - 1, b - 1, p - c);
    }
    let (res, dist) = bf.bellman_ford(0);
    if res || dist[n - 1] != -BellmanFord::INF {
        println!("{}", (-dist[n - 1]).max(0));
    } else {
        println!("{}", -1);
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

#[derive(Debug, Clone)]
pub struct BellmanFord {
    size: usize,
    edge: Vec<Edge>,
}

impl BellmanFord {
    const INF: Cost = Cost::MAX / 2;

    pub fn new(size: usize) -> Self {
        Self { size, edge: vec![] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.edge.push(Edge::new(from, to, cost));
    }

    pub fn bellman_ford(&mut self, s: usize) -> (bool, Vec<Cost>) {
        let mut dist = vec![Self::INF; self.size];
        dist[s] = 0;
        for _ in 0..self.size {
            let mut update = false;
            for edge in &self.edge {
                if dist[edge.from] == Self::INF {
                    continue;
                }
                if dist[edge.from] + edge.cost < dist[edge.to] {
                    dist[edge.to] = dist[edge.from] + edge.cost;
                    update = true;
                }
            }
            if !update {
                return (true, dist);
            }
        }
        for _ in 0..self.size {
            for edge in &self.edge {
                if dist[edge.from] == Self::INF {
                    continue;
                }
                if dist[edge.from] + edge.cost < dist[edge.to] {
                    dist[edge.to] = -Self::INF;
                }
            }
        }
        (false, dist)
    }
}

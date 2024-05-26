use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        input! {
            query: usize,
            u: usize,
            v: usize,
        }
        match query {
            1 => {
                uf.merge(u - 1, v - 1);
            }
            2 => {
                println!("{}", if uf.same(u - 1, v - 1) { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect::<Vec<usize>>(),
            siz: vec![1; n],
        }
    }

    fn merge(&mut self, a: usize, b: usize) -> bool {
        let a = self.leader(a);
        let b = self.leader(b);
        if a == b {
            return false;
        }
        if self.siz[a] > self.siz[b] {
            self.par[b] = a;
            self.siz[a] += self.siz[b];
        } else {
            self.par[a] = b;
            self.siz[b] += self.siz[a];
        }
        true
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }

    fn leader(&mut self, a: usize) -> usize {
        if self.par[a] == a {
            return a;
        }
        self.par[a] = self.leader(self.par[a]);
        self.par[a]
    }
}

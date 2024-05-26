use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); k],
    }
    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        uf.merge(a - 1, b - 1);
    }
    let mut res = vec![0; n];
    for i in 0..n {
        res[i] = uf.size(i) - 1;
    }
    for &(a, b) in &ab {
        if uf.same(a - 1, b - 1) {
            res[a - 1] -= 1;
            res[b - 1] -= 1;
        }
    }
    for &(c, d) in &cd {
        if uf.same(c - 1, d - 1) {
            res[c - 1] -= 1;
            res[d - 1] -= 1;
        }
    }
    println!("{}", res.iter().join(" "));
}

#[derive(Debug, Clone)]
pub struct UnionFind {
    n: usize,
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            par: (0..n).collect::<Vec<usize>>(),
            siz: vec![1; n],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
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

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    pub fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.par[a] == a {
            return a;
        }
        self.par[a] = self.leader(self.par[a]);
        self.par[a]
    }

    pub fn size(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        let a = self.leader(a);
        self.siz[a]
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for i in 0..self.n {
            res[self.leader(i)].push(i);
        }
        res.into_iter()
            .filter(|f| !f.is_empty())
            .collect::<Vec<_>>()
    }
}

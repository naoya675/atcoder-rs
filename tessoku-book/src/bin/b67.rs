use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, usize, i32); m],
    }
    abc.sort_by_key(|f| -f.2);
    let mut uf = UnionFind::new(n);
    let mut res = 0;
    for (a, b, c) in abc {
        if !uf.same(a - 1, b - 1) {
            uf.merge(a - 1, b - 1);
            res += c;
        }
    }
    println!("{}", res);
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

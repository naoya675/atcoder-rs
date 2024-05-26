use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        q: usize,
    }
    let mut check = vec![true; m];
    let mut query = vec![];
    for _ in 0..q {
        input! {
            que: usize,
        }
        match que {
            1 => {
                input! { x: usize, }
                query.push((que, x - 1, 0));
                check[x - 1] = false;
            }
            2 => {
                input! { u: usize, v: usize, }
                query.push((que, u - 1, v - 1));
            }
            _ => unreachable!(),
        }
    }
    let mut uf = UnionFind::new(n);
    for (i, &(a, b)) in ab.iter().enumerate() {
        if check[i] {
            uf.merge(a - 1, b - 1);
        }
    }
    let mut res = vec![];
    for &(query, u, v) in query.iter().rev() {
        match query {
            1 => {
                let (a, b) = ab[u];
                uf.merge(a - 1, b - 1);
            }
            2 => {
                res.push(uf.same(u, v));
            }
            _ => unreachable!(),
        }
    }
    for &res in res.iter().rev() {
        println!("{}", if res { "Yes" } else { "No" })
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

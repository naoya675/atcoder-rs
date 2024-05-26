use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        ac: [(usize, i64); n],
    }
    let inf = 1_000_000_000_000_000_000;
    let mut dp = vec![inf; l];
    for (a, c) in ac {
        dp[a - 1] = dp[a - 1].min(c);
    }
    let mut st = SegmentTree::<i64>::new(l, |a, b| min(a, b), inf);
    for i in 0..l {
        st.set(i, dp[i]);
    }
    let res = (0..l - k).map(|i| st.prod(i, i + k)).collect::<Vec<_>>();
    if res.iter().any(|&f| f == inf) {
        println!("-1");
    } else {
        println!("{}", res.iter().sum::<i64>());
    }
}

#[derive(Debug, Clone)]
struct SegmentTree<T> {
    tree: Vec<T>,
    size: usize,
    op: fn(T, T) -> T, // evaluation funciton
    e: T,              // identity element
}

impl<T: Copy> SegmentTree<T> {
    fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self {
        let size = n.next_power_of_two();
        Self {
            tree: vec![e; 2 * size],
            size,
            op,
            e,
        }
    }

    fn set(&mut self, mut k: usize, x: T) {
        k += self.size;
        self.tree[k] = x;
        while k > 0 {
            k /= 2;
            self.tree[k] = (self.op)(self.tree[2 * k], self.tree[2 * k + 1]);
        }
    }

    fn prod(&mut self, mut l: usize, mut r: usize) -> T {
        l += self.size;
        r += self.size;
        let mut res = self.e;
        while l < r {
            if l % 2 == 1 {
                res = (self.op)(res, self.tree[l]);
                l += 1;
            }
            l /= 2;
            if r % 2 == 1 {
                res = (self.op)(res, self.tree[r - 1]);
                r -= 1;
            }
            r /= 2;
        }
        res
    }
}

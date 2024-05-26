use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut st = SegmentTree::<i32>::new(n, |a, b| max(a, b), 0);
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    pos: usize,
                    x: i32,
                }
                st.set(pos - 1, x);
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                println!("{}", st.prod(l - 1, r - 1));
            }
            _ => unreachable!(),
        }
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

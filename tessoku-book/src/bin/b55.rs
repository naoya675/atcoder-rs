use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
        query: [(usize, i32); q],
    }
    let mut set = BTreeSet::new();
    for (query, x) in query {
        match query {
            1 => {
                set.insert(x);
            }
            2 => {
                let mut res = 1_000_000_000;
                if let Some(v) = set.range(..x).next_back() {
                    res = res.min((x - v).abs());
                }
                if let Some(v) = set.range(x..).next() {
                    res = res.min((x - v).abs());
                }
                println!("{}", if res == 1_000_000_000 { -1 } else { res });
            }
            _ => unreachable!(),
        }
    }
}

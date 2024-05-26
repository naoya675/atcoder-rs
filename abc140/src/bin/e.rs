use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut p = p
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i + 1))
        .collect::<Vec<_>>();
    p.sort();
    p.reverse();
    let mut set = BTreeSet::new();
    let mut res = 0;
    for (p, i) in p {
        let a = *set.range(..i).next_back().unwrap_or(&0);
        let b = *set.range(..a).next_back().unwrap_or(&0);
        let c = *set.range(i + 1..).next().unwrap_or(&(n + 1));
        let d = *set.range(c + 1..).next().unwrap_or(&(n + 1));
        res += (a - b) * (c - i) * p;
        res += (d - c) * (i - a) * p;
        set.insert(i);
    }
    println!("{}", res);
}

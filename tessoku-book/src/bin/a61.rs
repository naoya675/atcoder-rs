use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n + 1];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    for i in 1..=n {
        println!("{}: {{{}}}", i, graph[i].iter().join(", "));
    }
}

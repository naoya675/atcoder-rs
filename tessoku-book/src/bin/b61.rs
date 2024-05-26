use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let res = (0..n).max_by_key(|&f| graph[f].len()).unwrap();
    // for i in 0..n {
    //     if graph[res].len() < graph[i].len() {
    //         res = i;
    //     }
    // }
    println!("{}", res + 1);
}

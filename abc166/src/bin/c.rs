use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [i32; n],
        ab: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut res = 0;
    for i in 0..n {
        if graph[i].iter().all(|&j| h[i] > h[j]) {
            res += 1;
        }
    }
    println!("{}", res);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(usize, usize, usize, i32); q],
    }
    let mut vec = vec![0; n];
    let res = dfs(n, m, 0, 0, &abcd, &mut vec);
    println!("{}", res);
}

fn dfs(
    n: usize,
    m: usize,
    nc: usize,
    mc: usize,
    abcd: &Vec<(usize, usize, usize, i32)>,
    vec: &mut Vec<usize>,
) -> i32 {
    if nc == n {
        let mut res = 0;
        for &(a, b, c, d) in abcd {
            if vec[b - 1] - vec[a - 1] == c {
                res += d;
            }
        }
        return res;
    }
    let mut res = 0;
    for mc in mc..m {
        vec[nc] = mc;
        res = res.max(dfs(n, m, nc + 1, mc, abcd, vec));
    }
    res
}

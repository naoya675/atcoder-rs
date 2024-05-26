use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[i32; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    let mut vec = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            vec[i + 1][j + 1] = vec[i + 1][j] + vec[i][j + 1] - vec[i][j] + x[i][j];
        }
    }
    for (a, b, c, d) in abcd {
        let res = vec[c][d] - vec[a - 1][d] - vec[c][b - 1] + vec[a - 1][b - 1];
        println!("{}", res);
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    let mut vec = vec![vec![0; 2002]; 2002];
    for (x, y) in xy {
        vec[x][y] += 1;
    }
    for i in 0..2000 {
        for j in 0..2000 {
            vec[i][j + 1] += vec[i][j];
        }
    }
    for i in 0..2000 {
        for j in 0..2000 {
            vec[i + 1][j] += vec[i][j];
        }
    }
    for (a, b, c, d) in abcd {
        let res = vec[c][d] - vec[a - 1][d] - vec[c][b - 1] + vec[a - 1][b - 1];
        println!("{}", res);
    }
}

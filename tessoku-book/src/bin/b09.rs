use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut vec = vec![vec![0; 2002]; 2002];
    for (a, b, c, d) in abcd {
        vec[a][b] += 1;
        vec[a][d] -= 1;
        vec[c][b] -= 1;
        vec[c][d] += 1;
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
    let mut res = 0;
    for i in 0..2000 {
        for j in 0..2000 {
            if vec[i][j] > 0 {
                res += 1;
            }
        }
    }
    println!("{}", res);
}

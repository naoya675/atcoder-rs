use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut vec = vec![vec![0; w + 1]; h + 1];
    for (a, b, c, d) in abcd {
        vec[a - 1][b - 1] += 1;
        vec[a - 1][d] -= 1;
        vec[c][b - 1] -= 1;
        vec[c][d] += 1;
    }
    for i in 0..h {
        for j in 0..w {
            vec[i + 1][j] += vec[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            vec[i][j + 1] += vec[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{} ", vec[i][j]);
        }
        println!()
    }
}

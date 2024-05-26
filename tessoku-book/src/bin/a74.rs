use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [[usize; n]; n],
    }
    let mut col = vec![0; n];
    let mut row = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            col[i] = col[i].max(p[i][j]);
            row[j] = row[j].max(p[i][j]);
        }
    }
    let mut res = 0;
    for i in 0..n {
        for j in i..n {
            if col[i] > col[j] {
                res += 1;
            }
            if row[i] > row[j] {
                res += 1;
            }
        }
    }
    println!("{}", res);
}

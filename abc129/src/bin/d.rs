use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut l = vec![vec![0; w]; h];
    let mut r = vec![vec![0; w]; h];
    let mut d = vec![vec![0; w]; h];
    let mut u = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                l[i][j] = if j == 0 { 1 } else { l[i][j - 1] + 1 };
            }
        }
        for j in (0..w).rev() {
            if s[i][j] == '.' {
                r[i][j] = if j == w - 1 { 1 } else { r[i][j + 1] + 1 };
            }
        }
    }
    for j in 0..w {
        for i in 0..h {
            if s[i][j] == '.' {
                d[i][j] = if i == 0 { 1 } else { d[i - 1][j] + 1 };
            }
        }
        for i in (0..h).rev() {
            if s[i][j] == '.' {
                u[i][j] = if i == h - 1 { 1 } else { u[i + 1][j] + 1 };
            }
        }
    }
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                res = res.max(l[i][j] + r[i][j] + d[i][j] + u[i][j] - 3);
            }
        }
    }
    println!("{}", res);
}

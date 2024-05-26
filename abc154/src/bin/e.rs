use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
        k: usize,
    }
    let n = n
        .iter()
        .map(|&n| n.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let mut dp0 = vec![vec![0_i64; k + 2]; n.len() + 1];
    let mut dp1 = vec![vec![0_i64; k + 2]; n.len() + 1];
    dp0[0][0] = 1;
    for i in 0..n.len() {
        for j in 0..=k {
            for k in 0..10 {
                let count = if k == 0 { 0 } else { 1 };
                dp1[i + 1][j + count] += dp1[i][j];
            }
            for k in 0..n[i] + 1 {
                let count = if k == 0 { 0 } else { 1 };
                if k == n[i] {
                    dp0[i + 1][j + count] += dp0[i][j];
                } else {
                    dp1[i + 1][j + count] += dp0[i][j];
                }
            }
        }
    }
    println!("{}", dp0[n.len()][k] + dp1[n.len()][k]);
}

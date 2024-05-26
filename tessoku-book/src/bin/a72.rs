use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }
    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                cnt += 1;
            }
        }
    }
    let mut res = 0;
    for bit in 0..1 << h {
        let mut cnt = cnt;
        let mut row = vec![0; w];
        let mut sel = 0;
        for i in 0..h {
            if bit & (1 << i) != 0 {
                sel += 1;
                for j in 0..w {
                    if c[i][j] == '.' {
                        cnt += 1;
                    }
                }
            } else {
                for j in 0..w {
                    if c[i][j] == '.' {
                        row[j] += 1;
                    }
                }
            }
        }
        if sel > k {
            continue;
        }
        row.sort();
        row.reverse();
        for i in 0..k - sel {
            cnt += row[i];
        }
        res = res.max(cnt);
    }
    println!("{}", res);
}

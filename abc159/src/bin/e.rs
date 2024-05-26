use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }
    let mut res = 1 << 30;
    'outer: for bit in 0..1 << (h - 1) {
        let mut count = vec![vec![0; w]; h];
        let mut count_pop = 0;
        for i in 0..h {
            for j in 0..w {
                if s[i][j] == '1' {
                    count[count_pop][j] += 1;
                }
            }
            if bit & (1 << i) != 0 {
                count_pop += 1;
            }
        }
        count_pop += 1;
        let mut chocolate = vec![0; count_pop];
        let mut sep = vec![];
        let mut pre = 0;
        for j in 0..w {
            let mut ng = false;
            for i in 0..count_pop {
                if chocolate[i] + count[i][j] > k {
                    ng = true;
                }
            }
            if ng {
                if pre == j {
                    continue 'outer;
                }
                pre = j;
                sep.push(j);
                for i in 0..count_pop {
                    chocolate[i] = 0;
                }
            }
            for i in 0..count_pop {
                chocolate[i] += count[i][j];
            }
        }
        count_pop -= 1;
        res = res.min(count_pop + sep.len());
    }
    println!("{}", res);
}

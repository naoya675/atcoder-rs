use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut switch = vec![vec![0; n]; m];
    for i in 0..m {
        input! {
            k: usize,
            s: [usize; k],
        }
        for s in s {
            switch[i][s - 1] = 1;
        }
    }
    input! {
        p: [usize; m],
    }
    let mut res = 0;
    for bit in 0..(1 << n) {
        let mut switch_on = vec![0; m];
        for i in 0..n {
            if bit & (1 << i) != 0 {
                for j in 0..m {
                    switch_on[j] += switch[j][i];
                }
            }
        }
        if switch_on.iter().enumerate().all(|(i, s)| s % 2 == p[i]) {
            res += 1;
        }
    }
    println!("{}", res);
}

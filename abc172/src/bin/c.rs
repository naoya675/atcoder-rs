use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        a: [i64; n],
        b: [i64; m],
    }
    let mut acc = vec![0; n + 1];
    let mut bcc = vec![0; m + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    for i in 0..m {
        bcc[i + 1] = bcc[i] + b[i];
    }
    let mut res = 0;
    let mut j = m;
    for i in (0..n + 1).take_while(|&i| acc[i] <= k) {
        while acc[i] + bcc[j] > k {
            j -= 1;
        }
        res = res.max(i + j);
    }
    println!("{}", res);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [f64; n],
    }
    let mut acc = vec![0.; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + (p[i] + 1.) / 2.;
    }
    let mut res: f64 = 0.;
    for i in k..=n {
        res = res.max(acc[i] - acc[i - k]);
    }
    println!("{}", res);
}

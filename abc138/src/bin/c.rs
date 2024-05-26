use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [f64; n],
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut res = v[0];
    for i in 1..n {
        res = (res + v[i]) / 2.;
    }
    println!("{}", res);
}

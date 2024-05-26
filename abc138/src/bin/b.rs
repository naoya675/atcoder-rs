use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n],
    }
    let res = 1. / a.iter().map(|a| 1. / a).sum::<f64>();
    println!("{}", res);
}

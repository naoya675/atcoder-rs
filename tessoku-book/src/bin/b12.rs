use proconio::input;

fn main() {
    input! {
        n: f64,
    }
    let mut ok = n;
    let mut ng = 0.;
    while ok - ng > 1e-6 {
        let mi = (ok + ng) / 2.;
        if mi * mi * mi + mi < n {
            ng = mi;
        } else {
            ok = mi;
        }
    }
    println!("{}", ok);
}

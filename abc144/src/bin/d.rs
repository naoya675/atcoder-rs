use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }
    let mut ok = std::f64::consts::PI / 2.;
    let mut ng = 0.;
    while ok - ng > 1e-9 {
        let mi = (ok + ng) / 2.;
        if f(a, b, mi) < x {
            ok = mi;
        } else {
            ng = mi;
        }
    }
    println!("{}", ok / std::f64::consts::PI * 180.);
}

fn f(a: f64, b: f64, theta: f64) -> f64 {
    if a * theta.tan() <= b {
        a * a * b - a * a * a * theta.tan() / 2.
    } else {
        b * b * a / theta.tan() / 2.
    }
}

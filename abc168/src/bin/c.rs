use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    // cosine formula
    let rad = 2. * std::f64::consts::PI * ((h + m / 60.) / 12. - m / 60.).abs();
    let res = (a * a + b * b - 2. * a * b * rad.cos()).sqrt();
    println!("{}", res);
}

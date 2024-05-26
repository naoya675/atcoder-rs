use proconio::input;

fn main() {
    input! {
        r: f64,
    }
    println!("{}", 2. * std::f64::consts::PI * r);
}

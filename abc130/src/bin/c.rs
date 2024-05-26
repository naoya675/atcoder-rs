use proconio::input;

fn main() {
    input! {
        w: f64,
        h: f64,
        x: f64,
        y: f64,
    }
    let area = h * w / 2.;
    if w == x * 2. && h == y * 2. {
        println!("{} {}", area, 1);
    } else {
        println!("{} {}", area, 0);
    }
}

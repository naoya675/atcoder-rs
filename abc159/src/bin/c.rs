use proconio::input;

fn main() {
    input! {
        l: f64,
    }
    let l = l / 3.;
    println!("{}", l * l * l);
}

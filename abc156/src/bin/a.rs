use proconio::input;

fn main() {
    input! {
        n: i32,
        r: i32,
    }
    println!("{}", if n < 10 { r + 100 * (10 - n) } else { r });
}

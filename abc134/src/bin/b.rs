use proconio::input;

fn main() {
    input! {
        n: i32,
        d: i32,
    }
    println!("{}", (2 * d + n) / (2 * d + 1));
}

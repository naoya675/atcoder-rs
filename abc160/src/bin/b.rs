use proconio::input;

fn main() {
    input! {
        x: i32,
    }
    println!("{}", x / 500 * 1000 + (x % 500) / 5 * 5);
}

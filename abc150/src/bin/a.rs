use proconio::input;

fn main() {
    input! {
        k: i32,
        x: i32,
    }
    println!("{}", if 500 * k >= x { "Yes" } else { "No" });
}

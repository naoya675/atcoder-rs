use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!("{}", (b - 1 + a - 1 - 1) / (a - 1));
}

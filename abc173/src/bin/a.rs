use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    println!("{}", (1000 - n % 1000) % 1000);
}

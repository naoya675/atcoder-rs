use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    for i in 0..10 {
        print!("{}", (n >> 9 - i) % 2);
    }
    println!();
}

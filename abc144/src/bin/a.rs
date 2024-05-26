use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!("{}", if a < 10 && b < 10 { a * b } else { -1 });
}

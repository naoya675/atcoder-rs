use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    if a <= 5 {
        println!("{}", 0);
    } else if a <= 12 {
        println!("{}", b / 2);
    } else {
        println!("{}", b);
    }
}

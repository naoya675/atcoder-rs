use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,

    }
    if y % 2 == 0 && 2 * x <= y && y <= 4 * x {
        println!("Yes");
    } else {
        println!("No");
    }
}

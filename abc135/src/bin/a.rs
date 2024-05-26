use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    if (a - b).abs() % 2 == 0 {
        println!("{}", (a + b).abs() / 2);
    } else {
        println!("IMPOSSIBLE");
    }
}

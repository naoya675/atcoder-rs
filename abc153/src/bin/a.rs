use proconio::input;

fn main() {
    input! {
        h: i32,
        a: i32,
    }
    println!("{}", (h + a - 1) / a);
}

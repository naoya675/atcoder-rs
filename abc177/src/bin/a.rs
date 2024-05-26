use proconio::input;

fn main() {
    input! {
        d: i32,
        t: i32,
        s: i32,
    }
    println!("{}", if d <= s * t { "Yes" } else { "No" });
}

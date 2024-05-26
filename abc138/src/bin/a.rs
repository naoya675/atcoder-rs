use proconio::input;

fn main() {
    input! {
        a: i32,
        s: String,
    }
    println!("{}", if a < 3200 { "red".to_string() } else { s });
}

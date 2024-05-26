use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
        a: i32,
        b: i32,
        u: String,
    }
    if s == u {
        println!("{} {}", a - 1, b);
    }
    if t == u {
        println!("{} {}", a, b - 1);
    }
}

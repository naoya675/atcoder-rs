use proconio::input;

fn main() {
    input! {
        mut a: i32,
        b: i32,
        mut c: i32,
        d: i32,
    }
    loop {
        c -= b;
        if c <= 0 {
            break;
        }
        a -= d;
        if a <= 0 {
            break;
        }
    }
    println!("{}", if a > 0 { "Yes" } else { "No" });
}

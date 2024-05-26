use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    let mut res = false;
    if a == b && a != c {
        res = true;
    }
    if b == c && b != a {
        res = true;
    }
    if c == a && c != b {
        res = true;
    }
    println!("{}", if res { "Yes" } else { "No" });
}

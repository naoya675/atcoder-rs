use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let mut res = false;
    for i in a..=b {
        if 100 % i == 0 {
            res = true;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}

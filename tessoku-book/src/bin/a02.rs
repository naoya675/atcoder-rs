use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        a: [i32; n],
    }
    let mut res = false;
    for ai in a {
        if ai == x {
            res = true;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}

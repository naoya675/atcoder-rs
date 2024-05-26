use proconio::input;

fn main() {
    input! {
        k: i32,
        a: i32,
        b: i32,
    }
    let res = (a..=b).any(|i| i % k == 0);
    println!("{}", if res { "OK" } else { "NG" });
}

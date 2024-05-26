use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let mut res = false;
    for i in 1..10 {
        for j in 1..10 {
            if i * j == n {
                res = true;
            }
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}

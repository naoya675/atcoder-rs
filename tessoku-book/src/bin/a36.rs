use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }
    if n + n - 2 <= k && k % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

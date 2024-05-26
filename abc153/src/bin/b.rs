use proconio::input;

fn main() {
    input! {
        h: i32,
        n: usize,
        a: [i32; n],
    }
    let sum = a.iter().sum::<i32>();
    println!("{}", if h <= sum { "Yes" } else { "No" });
}

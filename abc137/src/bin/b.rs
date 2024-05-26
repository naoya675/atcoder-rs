use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        k: i32,
        x: i32,
    }
    let res = (-k + 1..k).map(|i| i + x).collect::<Vec<_>>();
    println!("{}", res.iter().join(" "));
}

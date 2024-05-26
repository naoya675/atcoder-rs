use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let acc = a.iter().fold(0, |acc, &a| acc ^ a);
    println!("{}", a.iter().map(|&a| acc ^ a).join(" "));
}

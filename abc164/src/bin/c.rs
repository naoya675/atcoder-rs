use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut set = BTreeSet::new();
    for s in s {
        set.insert(s);
    }
    println!("{}", set.len());
}

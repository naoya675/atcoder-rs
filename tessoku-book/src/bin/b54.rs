use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    let mut res = 0_usize;
    for ai in a {
        if let Some(v) = map.get(&ai) {
            res += v;
        }
        *map.entry(ai).or_default() += 1;
    }
    println!("{}", res);
}

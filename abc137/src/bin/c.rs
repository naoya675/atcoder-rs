use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut map: HashMap<Vec<char>, usize> = HashMap::new();
    let mut res = 0;
    for s in s {
        let mut s = s.clone();
        s.sort();
        if let Some(v) = map.get(&s) {
            res += v;
        }
        *map.entry(s).or_default() += 1;
    }
    println!("{}", res);
}

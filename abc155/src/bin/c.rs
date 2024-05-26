use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut map: HashMap<String, i32> = HashMap::new();
    for s in s {
        *map.entry(s).or_default() += 1;
    }
    let max = map.values().max().unwrap();
    let mut res = vec![];
    for (s, c) in map.iter() {
        if max == c {
            res.push(s);
        }
    }
    res.sort();
    for s in res {
        println!("{}", s);
    }
}

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut map: HashMap<i64, i64> = HashMap::new();
    let mut res = 0;
    for i in 0..n {
        let l = i as i64 + a[i];
        let r = i as i64 - a[i];
        if let Some(v) = map.get(&r) {
            res += v;
        }
        *map.entry(l).or_default() += 1;
    }
    println!("{}", res);
}

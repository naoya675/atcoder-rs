use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! {
            query: usize,
            x: i32,
        }
        match query {
            1 => {
                set.insert(x);
            }
            2 => {
                set.remove(&x);
            }
            3 => {
                let res = match set.range(x..).next() {
                    Some(x) => x,
                    None => &-1,
                };
                println!("{}", res);
            }
            _ => unreachable!(),
        }
    }
}

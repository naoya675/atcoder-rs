use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        stx: [(i32, i32, i32); n],
        d: [i32; q],
    }
    let mut event = vec![];
    for (i, &(s, t, x)) in stx.iter().enumerate() {
        event.push((s - x, 0, (x, i)));
        event.push((t - x, 1, (x, i)));
    }
    for (i, &d) in d.iter().enumerate() {
        event.push((d, 2, (0, i)));
    }
    event.sort();
    let mut set = BTreeSet::new();
    for (_, e, (x, i)) in event {
        match e {
            0 => {
                set.insert((x, i));
            }
            1 => {
                set.remove(&(x, i));
            }
            2 => {
                let res = match set.iter().next() {
                    Some((x, _)) => x,
                    None => &-1,
                };
                println!("{}", res);
            }
            _ => unreachable!(),
        }
    }
}

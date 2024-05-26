use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        hw: [(usize, usize); m],
    }
    let mut hval = vec![0; h];
    let mut wval = vec![0; w];
    let mut hmax = vec![];
    let mut wmax = vec![];
    let mut hmaxval = 0;
    let mut wmaxval = 0;
    let mut set = BTreeSet::new();
    for (h, w) in hw {
        hval[h - 1] += 1;
        wval[w - 1] += 1;
        hmaxval = hmaxval.max(hval[h - 1]);
        wmaxval = wmaxval.max(wval[w - 1]);
        set.insert((h - 1, w - 1));
    }
    for i in 0..h {
        if hval[i] == hmaxval {
            hmax.push(i);
        }
    }
    for i in 0..w {
        if wval[i] == wmaxval {
            wmax.push(i);
        }
    }
    let res = hmaxval + wmaxval;
    for &h in &hmax {
        for &w in &wmax {
            if !set.contains(&(h, w)) {
                println!("{}", res);
                return;
            }
        }
    }
    println!("{}", res - 1);
}

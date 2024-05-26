use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let mut pi = 0;
    let mut qi = 0;
    for (i, per) in (1..n + 1).permutations(n).enumerate() {
        if p == per {
            pi = i;
        }
        if q == per {
            qi = i;
        }
    }
    println!("{}", pi.abs_diff(qi));
}

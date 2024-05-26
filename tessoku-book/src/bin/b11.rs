use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
        x: [i32; q],
    }
    a.sort();
    for xi in x {
        println!("{}", a.lower_bound(&xi));
    }
}
